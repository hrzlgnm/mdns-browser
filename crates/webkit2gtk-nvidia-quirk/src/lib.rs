// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! # webkit2gtk-nvidia-quirk
//!
//! A crate that provides session-aware workarounds for WebKitGTK rendering issues
//! on Linux systems with the proprietary NVIDIA driver.
//!
//! ## Problem
//!
//! When running WebKitGTK-based applications (such as Tauri apps) on Linux
//! with the proprietary NVIDIA driver, rendering issues occur that vary by session type:
//!
//! - **X11**: The DMABUF renderer causes blank windows
//! - **Wayland**: The application does not start
//!
//! Related upstream issues:
//! - [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
//! - [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)
//! - [WebKitGTK Bug #280210](https://bugs.webkit.org/show_bug.cgi?id=280210)
//!
//! ## Solution
//!
//! This crate detects the proprietary NVIDIA driver and the session type (X11/Wayland),
//! then applies the appropriate workaround:
//!
//! | Session Type | Workaround | Environment Variable |
//! |-------------|------------|---------------------|
//! | X11 | Disable DMABUF renderer | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
//! | Wayland | Disable NVIDIA explicit sync | `__NV_DISABLE_EXPLICIT_SYNC=1` |
//!
//! ## Wayland
//!
//! WebKitGTK's DMA-BUF renderer enables the Wayland explicit sync protocol on
//! the window surface but does not always set an acquire point, which
//! compositors enforce strictly and answer with a protocol error that kills the
//! connection (`Gdk-Message: Error 71 (Protocol error) dispatching to Wayland
//! display`). This is a WebKitGTK bug
//! ([#280210](https://bugs.webkit.org/show_bug.cgi?id=280210)) independent of
//! the NVIDIA driver version or the Wayland EGL library in use, so explicit
//! sync is disabled for every NVIDIA Wayland session.
//!
//! ## Detection Method
//!
//! The crate detects the NVIDIA driver by:
//! 1. If the primary/boot GPU (via `boot_display` or `boot_vga` attributes) has vendor ID 0x10de
//! 2. If any enumerated GPU's `device/driver` symlink (e.g.
//!    `/sys/class/drm/card0/device/driver`) resolves to a driver named `nvidia`
//!
//! GPU detection uses sysfs exclusively (`/sys/class/drm/`). This provides a simpler and
//! more reliable detection mechanism with no external runtime dependencies.
//!
//! This specifically targets the proprietary NVIDIA driver, not the open-source nouveau driver.
//!
//! ### Sandboxed environments (Flatpak)
//!
//! GPU and NVIDIA-driver detection reads are scoped to `/sys/class/drm`, which is one of the
//! sysfs subtrees Flatpak shares read-only with sandboxed apps by default (`/sys/block`,
//! `/sys/bus`, `/sys/class`, `/sys/dev`, `/sys/devices`). Earlier versions checked
//! `/sys/module/nvidia` directly, which is *not* part of that default allowlist and would
//! always be reported as missing inside a Flatpak sandbox, silently disabling the workaround.
//! Deriving driver detection from the `device/driver` symlink avoids that problem.
//!
//! Session type detection also has a sandbox-aware fallback (see below).
//!
//! ## Usage
//!
//! ```rust,no_run
//! use webkit2gtk_nvidia_quirk::{ApplyWorkaroundOptions, apply_workaround_with_options};
//!
//! let disable_dmabuf = std::env::args().any(|arg| arg == "--disable-dmabuf-renderer");
//! let disable_nv_sync = std::env::args().any(|arg| arg == "--disable-nv-explicit-sync");
//!
//! let options = ApplyWorkaroundOptions::default()
//!     .force_disable_dmabuf(disable_dmabuf)
//!     .force_disable_nv_explicit_sync(disable_nv_sync);
//!
//! apply_workaround_with_options(options);
//! ```
//!
//! ## API
//!
//! ### `is_primary_gpu_nvidia() -> bool`
//!
//! Checks whether the primary GPU is an NVIDIA GPU.
//!
//! Returns `true` if the primary GPU (boot_display or boot_vga attribute) has vendor ID 0x10de (NVIDIA),
//! Returns `false` otherwise. This function does not check kernel module loading.
//!
//! ### `needs_workaround() -> WorkaroundKind`
//!
//! Determines which workaround should be applied based on NVIDIA detection and session type.
//!
//! Returns `WorkaroundKind::None` if no workaround is needed, `WorkaroundKind::DisableWebkitDmabufRenderer`
//! for X11 sessions, or `WorkaroundKind::DisableNvExplicitSync` for Wayland sessions.
//!
//! ### `set_webkit_disable_dmabuf_renderer()`
//!
//! Sets the `WEBKIT_DISABLE_DMABUF_RENDERER` environment variable. Use this for X11 sessions.
//!
//! ### `nv_disable_explicit_sync()`
//!
//! Sets the `__NV_DISABLE_EXPLICIT_SYNC` environment variable. Use this for Wayland sessions.
//!
//! ### `apply_workaround_with_options(options: ApplyWorkaroundOptions)`
//!
//! Convenience function that applies workarounds based on the provided options.
//! If any force options are set, it applies those directly. Otherwise, it calls
//! [`needs_workaround`] to detect which workaround is needed.
//!
//! This is the recommended way to apply workarounds from CLI arguments.
//!
//! ### `WorkaroundKind`
//!
//! Enum representing the type of workaround to apply:
//! - `None`: No workaround needed
//! - `DisableWebkitDmabufRenderer`: Disable the DMABUF renderer (for X11)
//! - `DisableNvExplicitSync`: Disable NVIDIA explicit sync (for Wayland)
//!
//! ### `ApplyWorkaroundOptions`
//!
//! Builder struct for configuring which workarounds to force-apply.
//! Use the builder pattern to set options:
//! ```rust,no_run
//! use webkit2gtk_nvidia_quirk::{ApplyWorkaroundOptions, apply_workaround_with_options};
//!
//! let options = ApplyWorkaroundOptions::default()
//!     .force_disable_dmabuf(true);
//!
//! apply_workaround_with_options(options);
//! ```
//!
//! ## Platform Support
//!
//! This crate is Linux-only and provides no functionality on other platforms.

#![cfg(target_os = "linux")]

use std::path::Path;

#[derive(Debug)]
struct GpuDevice {
    is_primary: bool,
    is_nvidia: bool,
    uses_nvidia_driver: bool,
}

fn read_sysfs_file(path: &Path) -> Option<String> {
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
}

fn parse_vendor_id(card_path: &Path) -> u16 {
    let vendor_path = card_path.join("device/vendor");
    if let Some(content) = read_sysfs_file(&vendor_path) {
        return u16::from_str_radix(content.strip_prefix("0x").unwrap_or(&content), 16)
            .unwrap_or(0);
    }
    0
}

fn is_sysfs_attr_one(card_path: &Path, attr: &str) -> bool {
    read_sysfs_file(&card_path.join(attr)).as_deref() == Some("1")
}

/// Returns the name of the kernel driver bound to the GPU at `card_path`, if
/// any, by resolving the `device/driver` symlink (e.g. `.../drivers/nvidia`).
///
/// This only reads the symlink target text via `read_link`, which does not
/// require the target directory itself to be accessible - it works even in
/// sandboxes (such as Flatpak) that only expose `/sys/class`.
fn driver_name(card_path: &Path) -> Option<String> {
    let driver_link = card_path.join("device/driver");
    let target = std::fs::read_link(driver_link).ok()?;
    target.file_name()?.to_str().map(str::to_string)
}

/// Enumerates GPUs found under `drm_path` (typically `/sys/class/drm`).
fn enumerate_gpus_at(drm_path: &Path) -> Vec<GpuDevice> {
    let mut devices = Vec::new();

    let entries = match std::fs::read_dir(drm_path) {
        Ok(e) => e,
        Err(_) => return devices,
    };

    for entry in entries.flatten() {
        let file_name = entry.file_name();
        let sysname = match file_name.to_str() {
            Some(s) => s,
            None => continue,
        };

        if !sysname.starts_with("card") || sysname.contains('-') {
            continue;
        }

        let card_path = entry.path();
        let vendor_id = parse_vendor_id(&card_path);

        let is_primary = is_sysfs_attr_one(&card_path, "boot_display")
            || is_sysfs_attr_one(&card_path.join("device"), "boot_display")
            || is_sysfs_attr_one(&card_path.join("device"), "boot_vga");

        let is_nvidia = vendor_id == 0x10de;
        let uses_nvidia_driver = driver_name(&card_path).as_deref() == Some("nvidia");

        devices.push(GpuDevice {
            is_primary,
            is_nvidia,
            uses_nvidia_driver,
        });
    }

    devices
}

fn enumerate_gpus() -> Vec<GpuDevice> {
    enumerate_gpus_at(Path::new("/sys/class/drm"))
}

/// Returns whether any enumerated GPU is currently bound to the proprietary
/// `nvidia` kernel driver.
///
/// This is derived from the `device/driver` symlink of each GPU under
/// `/sys/class/drm` rather than checking for `/sys/module/nvidia`, since the
/// latter is not exposed inside sandboxes (such as Flatpak) that only share
/// `/sys/class` (and similar subtrees) with the app by default.
fn nvidia_driver_loaded(devices: &[GpuDevice]) -> bool {
    devices.iter().any(|d| d.uses_nvidia_driver)
}

#[derive(Debug, PartialEq, Eq)]
enum SessionType {
    Wayland,
    X11,
    Unknown,
}

/// Determines the session type from the relevant environment variables.
///
/// `XDG_SESSION_TYPE` is preferred when set to a recognized value. Sandboxed
/// environments such as Flatpak do not propagate `XDG_SESSION_TYPE` into the
/// sandbox, so `WAYLAND_DISPLAY` and `DISPLAY` are used as a fallback: Flatpak
/// sets these automatically when the corresponding socket permission
/// (`--socket=wayland`, `--socket=x11`/`--socket=fallback-x11`) is granted.
fn session_type_from_env(
    xdg_session_type: Option<&str>,
    wayland_display: Option<&str>,
    display: Option<&str>,
) -> SessionType {
    match xdg_session_type {
        Some("x11") => return SessionType::X11,
        Some("wayland") => return SessionType::Wayland,
        _ => {}
    }
    if wayland_display.is_some() {
        SessionType::Wayland
    } else if display.is_some() {
        SessionType::X11
    } else {
        SessionType::Unknown
    }
}

/// Detects the used session type based upon the `XDG_SESSION_TYPE` environment
/// variable, falling back to `WAYLAND_DISPLAY`/`DISPLAY` when unavailable (e.g.
/// inside a Flatpak sandbox). See [`session_type_from_env`] for details.
fn get_session_type() -> SessionType {
    session_type_from_env(
        std::env::var("XDG_SESSION_TYPE").ok().as_deref(),
        std::env::var("WAYLAND_DISPLAY").ok().as_deref(),
        std::env::var("DISPLAY").ok().as_deref(),
    )
}

/// Represents the type of workaround to apply for NVIDIA WebKitGTK issues.
///
/// Use this enum to determine which workaround is needed based on the session type
/// and whether NVIDIA is detected.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorkaroundKind {
    /// No workaround needed.
    None,
    /// Disable the WebKit DMABUF renderer.
    ///
    /// This workaround is needed for X11 sessions with NVIDIA drivers.
    DisableWebkitDmabufRenderer,
    /// Disable NVIDIA explicit sync.
    ///
    /// This workaround is needed for Wayland sessions with NVIDIA drivers.
    DisableNvExplicitSync,
}

/// Selects the workaround for a detected session and GPU state.
///
/// On Wayland the explicit sync workaround is applied unconditionally: the
/// protocol error is caused by WebKitGTK itself, not by the driver's Wayland
/// EGL library, so there is no configuration where it can be skipped.
fn workaround_for(session: SessionType, nvidia_detected: bool) -> WorkaroundKind {
    if !nvidia_detected {
        return WorkaroundKind::None;
    }
    match session {
        SessionType::Wayland => WorkaroundKind::DisableNvExplicitSync,
        SessionType::X11 => WorkaroundKind::DisableWebkitDmabufRenderer,
        SessionType::Unknown => WorkaroundKind::None,
    }
}

/// Checks if a workaround should be applied.
///
/// This function checks if the proprietary NVIDIA driver is loaded and the primary GPU is NVIDIA.
/// If so, it detects the session type (X11 or Wayland) and returns which workaround should be applied.
///
/// # Returns
///
///  `None` if no workaround is needed
///  `DisableWebkitDmabufRenderer` if disabling the dmabuf renderer should be applied
///  `DisableNvExplicitSync` if disabling nvidia explicit sync should be applied
///
/// # Note
///
/// This function only performs detection. Use [`set_webkit_disable_dmabuf_renderer`] or
/// [`nv_disable_explicit_sync`] to apply the respective workaround.
/// Call this first, then call the workaround if needed - ideally before spawning any threads.
pub fn needs_workaround() -> WorkaroundKind {
    let devices = enumerate_gpus();
    let nvidia_detected =
        devices.iter().any(|d| d.is_primary && d.is_nvidia) && nvidia_driver_loaded(&devices);
    workaround_for(get_session_type(), nvidia_detected)
}

/// Checks if the primary GPU is an NVIDIA GPU.
///
/// Returns `true` if the primary GPU (boot_display or boot_vga) is NVIDIA
/// Returns `false` otherwise.
pub fn is_primary_gpu_nvidia() -> bool {
    let devices = enumerate_gpus();

    devices.iter().any(|d| d.is_primary && d.is_nvidia)
}

/// Sets the `WEBKIT_DISABLE_DMABUF_RENDERER` environment variable.
///
/// This function should be called explicitly from single-threaded startup
/// (main) before spawning threads or when launching subprocesses.
///
/// # Note
///
/// This function modifies the process environment. Call it early in your
/// application's startup, before any threading has begun.
pub fn set_webkit_disable_dmabuf_renderer() {
    eprintln!("Note: disabling dmabuf renderer, expect degraded renderer performance.");
    eprintln!("See https://github.com/tauri-apps/tauri/issues/9304 for more details.");
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
}

/// Sets the `__NV_DISABLE_EXPLICIT_SYNC` environment variable.
///
/// This function should be called explicitly from single-threaded startup
/// (main) before spawning threads or when launching subprocesses.
///
/// # Note
///
/// This function modifies the process environment. Call it early in your
/// application's startup, before any threading has begun.
pub fn nv_disable_explicit_sync() {
    eprintln!("Note: disabling nvidia explicit sync.");
    eprintln!("See https://bugs.webkit.org/show_bug.cgi?id=280210 for more details");
    std::env::set_var("__NV_DISABLE_EXPLICIT_SYNC", "1");
}

/// Builder struct for configuring which workarounds to force-apply.
///
/// Use the builder pattern to set options before passing to [`apply_workaround_with_options`].
///
/// # Example
///
/// ```rust,no_run
/// use webkit2gtk_nvidia_quirk::{ApplyWorkaroundOptions, apply_workaround_with_options};
///
/// let options = ApplyWorkaroundOptions::default()
///     .force_disable_dmabuf(true)
///     .force_disable_nv_explicit_sync(true);
///
/// apply_workaround_with_options(options);
/// ```
#[derive(Default)]
pub struct ApplyWorkaroundOptions {
    /// Force disable the DMABUF renderer.
    pub force_disable_dmabuf: bool,
    /// Force disable NVIDIA explicit sync.
    pub force_disable_nv_explicit_sync: bool,
}

impl ApplyWorkaroundOptions {
    /// Sets the `force_disable_dmabuf` option.
    ///
    /// When `true`, the DMABUF renderer will be disabled regardless of
    /// whether NVIDIA is detected.
    pub fn force_disable_dmabuf(mut self, value: bool) -> Self {
        self.force_disable_dmabuf = value;
        self
    }

    /// Sets the `force_disable_nv_explicit_sync` option.
    ///
    /// When `true`, NVIDIA explicit sync will be disabled regardless of
    /// whether NVIDIA is detected.
    pub fn force_disable_nv_explicit_sync(mut self, value: bool) -> Self {
        self.force_disable_nv_explicit_sync = value;
        self
    }
}

/// Applies workarounds based on the provided options.
///
/// If any force options are set in `options`, those workarounds are applied directly.
/// Otherwise, it calls [`needs_workaround`] to detect which workaround is needed.
///
/// # Arguments
///
/// * `options` - The workaround options to apply
///
/// # Note
///
/// This function modifies the process environment. Call it early in your
/// application's startup, before any threading has begun.
pub fn apply_workaround_with_options(options: ApplyWorkaroundOptions) {
    if options.force_disable_dmabuf {
        set_webkit_disable_dmabuf_renderer();
    }
    if options.force_disable_nv_explicit_sync {
        nv_disable_explicit_sync();
    }
    if !options.force_disable_dmabuf && !options.force_disable_nv_explicit_sync {
        match needs_workaround() {
            WorkaroundKind::None => {}
            WorkaroundKind::DisableWebkitDmabufRenderer => set_webkit_disable_dmabuf_renderer(),
            WorkaroundKind::DisableNvExplicitSync => nv_disable_explicit_sync(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn temp_dir(name: &str) -> PathBuf {
        let dir = std::env::temp_dir().join(format!(
            "webkit2gtk-nvidia-quirk-test-{}-{}",
            name,
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// Creates a fake `/sys/class/drm/<name>` GPU entry with a `device/driver`
    /// symlink pointing at a driver named `driver`, and optionally a
    /// `boot_vga` attribute set to `1` and a `vendor` file.
    fn write_fake_card(
        drm_dir: &Path,
        name: &str,
        vendor: Option<&str>,
        driver: Option<&str>,
        boot_vga: bool,
    ) -> std::io::Result<PathBuf> {
        let card_path = drm_dir.join(name);
        let device_path = card_path.join("device");
        std::fs::create_dir_all(&device_path)?;

        if let Some(vendor) = vendor {
            std::fs::write(device_path.join("vendor"), vendor)?;
        }
        if boot_vga {
            std::fs::write(device_path.join("boot_vga"), "1")?;
        }
        if let Some(driver) = driver {
            // The real sysfs symlink target doesn't need to resolve to an
            // existing path for `read_link` to work - it only reads the
            // stored link text, matching how it behaves under sandboxes that
            // don't expose `/sys/bus`.
            let target = PathBuf::from(format!("../../../../bus/pci/drivers/{driver}"));
            std::os::unix::fs::symlink(target, device_path.join("driver"))?;
        }

        Ok(card_path)
    }

    #[test]
    fn test_driver_name_resolves_symlink() -> std::io::Result<()> {
        let dir = temp_dir("driver_name");
        let card = write_fake_card(&dir, "card0", Some("0x10de"), Some("nvidia"), true)?;
        assert_eq!(driver_name(&card), Some("nvidia".to_string()));
        Ok(())
    }

    #[test]
    fn test_driver_name_missing_symlink() -> std::io::Result<()> {
        let dir = temp_dir("driver_name_missing");
        let card = write_fake_card(&dir, "card0", Some("0x10de"), None, true)?;
        assert_eq!(driver_name(&card), None);
        Ok(())
    }

    #[test]
    fn test_enumerate_gpus_at_nvidia_primary() -> std::io::Result<()> {
        let dir = temp_dir("enumerate_nvidia_primary");
        write_fake_card(&dir, "card0", Some("0x10de"), Some("nvidia"), true)?;
        write_fake_card(&dir, "card1", Some("0x1002"), Some("amdgpu"), false)?;

        let devices = enumerate_gpus_at(&dir);
        assert_eq!(devices.len(), 2);
        assert!(devices
            .iter()
            .any(|d| d.is_primary && d.is_nvidia && d.uses_nvidia_driver));
        assert!(nvidia_driver_loaded(&devices));
        Ok(())
    }

    #[test]
    fn test_enumerate_gpus_at_nouveau_not_nvidia_driver() -> std::io::Result<()> {
        // NVIDIA vendor ID but the open-source nouveau driver bound - the
        // proprietary-driver check must not treat this as "loaded".
        let dir = temp_dir("enumerate_nouveau");
        write_fake_card(&dir, "card0", Some("0x10de"), Some("nouveau"), true)?;

        let devices = enumerate_gpus_at(&dir);
        assert!(devices.iter().any(|d| d.is_primary && d.is_nvidia));
        assert!(!nvidia_driver_loaded(&devices));
        Ok(())
    }

    #[test]
    fn test_enumerate_gpus_at_no_cards() {
        let dir = temp_dir("enumerate_empty");
        let devices = enumerate_gpus_at(&dir);
        assert!(devices.is_empty());
        assert!(!nvidia_driver_loaded(&devices));
    }

    #[test]
    fn test_session_type_from_env_prefers_xdg_session_type() {
        assert_eq!(
            session_type_from_env(Some("wayland"), None, Some(":0")),
            SessionType::Wayland
        );
        assert_eq!(
            session_type_from_env(Some("x11"), Some("wayland-0"), None),
            SessionType::X11
        );
    }

    #[test]
    fn test_session_type_from_env_falls_back_to_display_vars() {
        // Simulates a Flatpak sandbox where XDG_SESSION_TYPE isn't
        // propagated, but the socket permission env vars are set.
        assert_eq!(
            session_type_from_env(None, Some("wayland-0"), None),
            SessionType::Wayland
        );
        assert_eq!(
            session_type_from_env(None, None, Some(":0")),
            SessionType::X11
        );
        assert_eq!(
            session_type_from_env(None, None, None),
            SessionType::Unknown
        );
    }

    #[test]
    fn test_session_type_from_env_unrecognized_xdg_session_type_falls_back() {
        assert_eq!(
            session_type_from_env(Some("tty"), Some("wayland-0"), None),
            SessionType::Wayland
        );
    }

    #[test]
    fn test_workaround_for_wayland_disables_nv_explicit_sync() {
        assert_eq!(
            workaround_for(SessionType::Wayland, true),
            WorkaroundKind::DisableNvExplicitSync
        );
    }

    #[test]
    fn test_workaround_for_x11_disables_dmabuf_renderer() {
        assert_eq!(
            workaround_for(SessionType::X11, true),
            WorkaroundKind::DisableWebkitDmabufRenderer
        );
    }

    #[test]
    fn test_workaround_for_unknown_session_is_noop() {
        assert_eq!(
            workaround_for(SessionType::Unknown, true),
            WorkaroundKind::None
        );
    }

    #[test]
    fn test_workaround_for_no_nvidia_is_noop() {
        assert_eq!(
            workaround_for(SessionType::Wayland, false),
            WorkaroundKind::None
        );
        assert_eq!(
            workaround_for(SessionType::X11, false),
            WorkaroundKind::None
        );
    }

    /// Integration test: exercises the full GPU-detection pipeline
    /// (`enumerate_gpus_at` + `nvidia_driver_loaded`) against a fake sysfs
    /// tree that mimics what is actually visible inside a Flatpak sandbox,
    /// i.e. only `/sys/class/drm` is present - there is no
    /// `/sys/module/nvidia` and no `/proc/driver/nvidia/version`. This is a
    /// regression test for the bug where the workaround silently became a
    /// no-op in sandboxes because detection relied on `/sys/module/nvidia`.
    #[test]
    fn test_needs_workaround_detection_in_simulated_flatpak_sandbox() -> std::io::Result<()> {
        let dir = temp_dir("flatpak_sandbox");
        // Only /sys/class/drm exists in the sandbox; deliberately do not
        // create anything resembling /sys/module or /proc.
        write_fake_card(&dir, "card0", Some("0x10de"), Some("nvidia"), true)?;

        let devices = enumerate_gpus_at(&dir);
        let primary_gpu_is_nvidia = devices.iter().any(|d| d.is_primary && d.is_nvidia);
        let driver_loaded = nvidia_driver_loaded(&devices);

        assert!(
            primary_gpu_is_nvidia && driver_loaded,
            "NVIDIA should still be detected from /sys/class/drm alone, \
             without /sys/module or /proc access"
        );

        // The session type would typically come from WAYLAND_DISPLAY/DISPLAY
        // in a Flatpak sandbox, since XDG_SESSION_TYPE is not propagated.
        let session = session_type_from_env(None, Some("wayland-0"), None);
        assert_eq!(session, SessionType::Wayland);

        // Wayland + NVIDIA must always disable explicit sync, regardless of
        // the driver version or the EGL config directories reachable inside
        // the sandbox.
        assert_eq!(
            workaround_for(session, primary_gpu_is_nvidia && driver_loaded),
            WorkaroundKind::DisableNvExplicitSync
        );
        Ok(())
    }
}
