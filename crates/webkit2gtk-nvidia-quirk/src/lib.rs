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
//! - **Wayland (Hyprland)**: The DMABUF renderer violates the compositor's
//!   acquire-point rule (a protocol error that kills the client) and its NVIDIA
//!   EGL/GBM render path SIGSEVs during rendering
//! - **Wayland (other)**: The application may not start unless NVIDIA explicit
//!   sync is disabled
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
//! | Wayland (Hyprland) | Disable DMABUF renderer | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
//! | Wayland (other, `egl-wayland2`) | none | - |
//! | Wayland (other) | Disable NVIDIA explicit sync | `__NV_DISABLE_EXPLICIT_SYNC=1` |
//!
//! The session type is taken from `GDK_BACKEND` first (the backend GDK/WebKitGTK
//! actually selects, a colon/comma separated list where the first recognized
//! entry wins), then `XDG_SESSION_TYPE`, then `WAYLAND_DISPLAY`/`DISPLAY`.
//!
//! ## Wayland
//!
//! WebKitGTK's DMA-BUF renderer enables the Wayland explicit sync protocol on
//! the window surface but does not always set an acquire point before
//! committing. Compositors differ in how strictly they enforce this: Hyprland
//! answers with a protocol error that kills the connection (`Gdk-Message:
//! Error 71 (Protocol error) dispatching to Wayland display`, WebKitGTK bug
//! [#280210](https://bugs.webkit.org/show_bug.cgi?id=280210)), while others
//! such as niri tolerate the missing acquire point. The DMA-BUF renderer is
//! therefore disabled on Hyprland, which also avoids a separate NVIDIA EGL/GBM
//! SIGSEGV that occurs during rendering there. On compositors that tolerate the
//! missing acquire point, the dma-buf based `egl-wayland2` library (NVIDIA
//! driver 560 or newer) is treated as working and the workaround is skipped,
//! since disabling explicit sync would degrade rendering performance; on those
//! compositors without `egl-wayland2`, NVIDIA explicit sync is disabled.
//!
//! Detecting `egl-wayland2` mirrors the EGL loader logic: the external platform
//! JSON manifests in `/etc/egl/egl_external_platform.d` and
//! `/usr/share/egl/egl_external_platform.d` (or the directories/files given via
//! `__EGL_EXTERNAL_PLATFORM_CONFIG_DIRS`/`__EGL_EXTERNAL_PLATFORM_CONFIG_FILENAMES`)
//! are checked in load order against `/proc/driver/nvidia/version`. Inside a
//! Flatpak sandbox neither `/proc` nor the host `/etc`/`/usr` directories are
//! visible, so `egl-wayland2` is reported as not active and the workaround is
//! applied - safe, but not perf-optimal for sandboxed apps on a 560+ driver.
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

use std::path::{Path, PathBuf};

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

/// The NVIDIA Wayland EGL platform library referenced by an external platform manifest.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WaylandLib {
    /// The EGLStreams based `egl-wayland` library.
    EglWayland,
    /// The dma-buf based `egl-wayland2` library.
    EglWayland2,
}

/// Classifies an external platform library path as either the old or the new
/// NVIDIA Wayland library.
fn classify_wayland_lib(library_path: &str) -> Option<WaylandLib> {
    if library_path.contains("nvidia-egl-wayland2") {
        Some(WaylandLib::EglWayland2)
    } else if library_path.contains("nvidia-egl-wayland") {
        Some(WaylandLib::EglWayland)
    } else {
        None
    }
}

/// Extracts the `library_path` from an EGL external platform JSON manifest.
fn library_path_from_manifest(content: &str) -> Option<&str> {
    let key = "\"library_path\"";
    let rest = &content[content.find(key)? + key.len()..];
    let open = rest.find('"')?;
    let value = &rest[open + 1..];
    let close = value.find('"')?;
    Some(&value[..close])
}

/// Reads an EGL external platform JSON manifest and classifies the NVIDIA
/// Wayland library it references, if any.
fn wayland_lib_from_manifest(path: &Path) -> Option<WaylandLib> {
    let content = read_sysfs_file(path)?;
    let library_path = library_path_from_manifest(&content)?;
    classify_wayland_lib(library_path)
}

/// Returns the `.json` files of an EGL external platform config directory,
/// sorted by filename as the EGL loader would try them.
fn json_files_in_dir(dir: &Path) -> Vec<PathBuf> {
    let mut files: Vec<PathBuf> = match std::fs::read_dir(dir) {
        Ok(entries) => entries
            .flatten()
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "json"))
            .collect(),
        Err(_) => Vec::new(),
    };
    files.sort();
    files
}

/// Returns the EGL external platform JSON config files in the order the EGL
/// loader would try them, honoring the `__EGL_EXTERNAL_PLATFORM_CONFIG_FILENAMES`
/// and `__EGL_EXTERNAL_PLATFORM_CONFIG_DIRS` environment variables.
fn egl_external_platform_config_files() -> Vec<PathBuf> {
    if let Ok(files) = std::env::var("__EGL_EXTERNAL_PLATFORM_CONFIG_FILENAMES") {
        return files.split(':').map(PathBuf::from).collect();
    }
    let dirs: Vec<PathBuf> = match std::env::var("__EGL_EXTERNAL_PLATFORM_CONFIG_DIRS") {
        Ok(dirs) => dirs.split(':').map(PathBuf::from).collect(),
        Err(_) => vec![
            PathBuf::from("/etc/egl/egl_external_platform.d"),
            PathBuf::from("/usr/share/egl/egl_external_platform.d"),
        ],
    };
    dirs.iter().flat_map(|dir| json_files_in_dir(dir)).collect()
}

/// Returns the NVIDIA Wayland EGL library the loader would select first, given
/// the external platform JSON config files in load order.
fn first_wayland_lib(configs: &[PathBuf]) -> Option<WaylandLib> {
    configs
        .iter()
        .find_map(|config| wayland_lib_from_manifest(config))
}

/// Extracts the major version of a dotted NVIDIA driver version string, e.g.
/// `610` from `610.43.03`.
fn parse_driver_major(version: &str) -> Option<u64> {
    let bytes = version.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i].is_ascii_digit() {
            let start = i;
            while i < bytes.len() && bytes[i].is_ascii_digit() {
                i += 1;
            }
            let major = bytes[start..i]
                .iter()
                .fold(0u64, |acc, b| acc * 10 + u64::from(b - b'0'));
            if bytes.get(i) == Some(&b'.') && bytes.get(i + 1).is_some_and(|b| b.is_ascii_digit()) {
                return Some(major);
            }
        } else {
            i += 1;
        }
    }
    None
}

/// Returns the loaded NVIDIA driver major version, if it can be determined.
fn nvidia_driver_major() -> Option<u64> {
    let version = read_sysfs_file(Path::new("/proc/driver/nvidia/version"))?;
    parse_driver_major(&version)
}

/// Returns whether the dma-buf based `egl-wayland2` library is the one the EGL
/// loader would use for the Wayland platform.
///
/// This is the case when the first external platform manifest in load order
/// referencing an NVIDIA Wayland library points at `libnvidia-egl-wayland2.so`
/// and the loaded NVIDIA driver is version 560 or newer (the version that added
/// the driver interface `egl-wayland2` depends on). On older drivers the new
/// library fails to initialize and the loader falls back to the old one.
fn is_egl_wayland2_selected(configs: &[PathBuf], driver_major: Option<u64>) -> bool {
    matches!(first_wayland_lib(configs), Some(WaylandLib::EglWayland2))
        && driver_major.is_some_and(|major| major >= 560)
}

/// Reads the current system state and returns whether `egl-wayland2` is in use.
fn egl_wayland2_active() -> bool {
    let configs = egl_external_platform_config_files();
    let driver_major = nvidia_driver_major();
    is_egl_wayland2_selected(&configs, driver_major)
}

#[derive(Debug, PartialEq, Eq)]
enum SessionType {
    Wayland,
    X11,
    Unknown,
}

/// Parses `GDK_BACKEND` into the session type GDK would actually select.
///
/// GDK reads `GDK_BACKEND` as a colon/comma separated list of preferred
/// backends and uses the first one that is available, so the first recognized
/// entry wins. This is the backend WebKitGTK ends up using, which is why it
/// takes precedence over `XDG_SESSION_TYPE`/`WAYLAND_DISPLAY`/`DISPLAY`.
fn session_type_from_gdk_backend(gdk_backend: Option<&str>) -> Option<SessionType> {
    let backend = gdk_backend?
        .split([',', ':'])
        .map(str::trim)
        .find(|s| !s.is_empty())?;
    match backend {
        "x11" => Some(SessionType::X11),
        "wayland" => Some(SessionType::Wayland),
        _ => None,
    }
}

/// Determines the session type from the relevant environment variables.
///
/// `GDK_BACKEND` is consulted first, since it is the backend GDK/WebKitGTK
/// actually selects. `XDG_SESSION_TYPE` is preferred next when set to a
/// recognized value. Sandboxed environments such as Flatpak do not propagate
/// `XDG_SESSION_TYPE` into the sandbox, so `WAYLAND_DISPLAY` and `DISPLAY` are
/// used as a fallback: Flatpak sets these automatically when the corresponding
/// socket permission (`--socket=wayland`, `--socket=x11`/`--socket=fallback-x11`)
/// is granted.
fn session_type_from_env(
    gdk_backend: Option<&str>,
    xdg_session_type: Option<&str>,
    wayland_display: Option<&str>,
    display: Option<&str>,
) -> SessionType {
    if let Some(session) = session_type_from_gdk_backend(gdk_backend) {
        return session;
    }
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

/// Detects the used session type based upon the `GDK_BACKEND` and
/// `XDG_SESSION_TYPE` environment variables, falling back to
/// `WAYLAND_DISPLAY`/`DISPLAY` when unavailable (e.g. inside a Flatpak sandbox).
/// See [`session_type_from_env`] for details.
fn get_session_type() -> SessionType {
    session_type_from_env(
        std::env::var("GDK_BACKEND").ok().as_deref(),
        std::env::var("XDG_SESSION_TYPE").ok().as_deref(),
        std::env::var("WAYLAND_DISPLAY").ok().as_deref(),
        std::env::var("DISPLAY").ok().as_deref(),
    )
}

/// Returns `true` when the application is running in a Wayland session.
///
/// This reuses the same session detection as the WebKitGTK workarounds
/// ([`get_session_type`]), falling back to `WAYLAND_DISPLAY`/`DISPLAY` when
/// `XDG_SESSION_TYPE` is unavailable (e.g. inside a Flatpak sandbox).
///
/// It is used to apply Wayland-specific workarounds that must not run under X11.
pub fn is_wayland_session() -> bool {
    get_session_type() == SessionType::Wayland
}

/// Returns `true` when the application is running in an X11 session.
///
/// This reuses the same session detection as the WebKitGTK workarounds
/// ([`get_session_type`]), falling back to `WAYLAND_DISPLAY`/`DISPLAY` when
/// `XDG_SESSION_TYPE` is unavailable (e.g. inside a Flatpak sandbox).
///
/// It is used to apply X11-specific workarounds that must not run under Wayland.
pub fn is_x11_session() -> bool {
    get_session_type() == SessionType::X11
}

/// Compositors that lay out windows without traditional title bars, so client
/// side decorations are unwanted. This is a heuristic list since Wayland has no
/// protocol that reports tiling vs floating behavior. It is not exhaustive and
/// is matched against the desktop name advertised via
/// `XDG_CURRENT_DESKTOP`/`XDG_SESSION_DESKTOP`; some floating-first compositors
/// (e.g. Wayfire, KWin, GNOME Shell) only tile via optional plugins/extensions
/// and are intentionally excluded.
const TILING_COMPOSITORS: &[&str] = &[
    "Hyprland",
    "sway",
    "river",
    "niri",
    "dwl",
    "newm",
    "karuiwm",
    "japokwm",
    "qtile",
    "miraclewm",
    "vivarium",
    "waymonad",
];

/// Returns `true` when the running compositor is known to use a tiling layout.
///
/// Wayland provides no protocol to query tiling vs floating behavior, so this
/// matches the compositor advertised via `XDG_CURRENT_DESKTOP` (or
/// `XDG_SESSION_DESKTOP`) against a known set. The desktop-name comparison is
/// case-insensitive and tolerates `XDG_CURRENT_DESKTOP` being a
/// colon/comma/semicolon-separated list, as some sessions advertise more than
/// one desktop name.
///
/// It is used to skip client side decorations on compositors that do not render
/// or make use of them.
pub fn is_tiling_compositor() -> bool {
    let matches = |desktop: &str| -> bool {
        desktop.split([':', ';', ',']).map(str::trim).any(|part| {
            TILING_COMPOSITORS
                .iter()
                .any(|t| t.eq_ignore_ascii_case(part))
        })
    };
    get_compositor().as_deref().map(matches).unwrap_or(false)
}

/// Returns the compositor advertised in the environment, if any.
///
/// Compositors advertise their identity via `XDG_CURRENT_DESKTOP`, which is the
/// standard signal toolkits use to branch on compositor behavior.
/// `XDG_SESSION_DESKTOP` is used as a fallback since some compositors only set
/// that one.
fn compositor_from_env<'a>(
    xdg_current_desktop: Option<&'a str>,
    xdg_session_desktop: Option<&'a str>,
) -> Option<&'a str> {
    let current = xdg_current_desktop.filter(|s| !s.trim().is_empty());
    current.or(xdg_session_desktop)
}

/// Detects the running compositor from the environment. See
/// [`compositor_from_env`] for details.
fn get_compositor() -> Option<String> {
    compositor_from_env(
        std::env::var("XDG_CURRENT_DESKTOP").ok().as_deref(),
        std::env::var("XDG_SESSION_DESKTOP").ok().as_deref(),
    )
    .map(str::to_string)
}

/// Returns whether the running compositor is Hyprland.
///
/// Hyprland strictly enforces Wayland semantics that WebKitGTK's DMA-BUF
/// renderer violates: the renderer enables the explicit sync protocol on the
/// window surface but does not always set an acquire point before committing
/// (a protocol error that kills the client on Hyprland, tolerated by e.g.
/// niri), and its NVIDIA EGL/GBM render path SIGSEVs during rendering
/// (`libnvidia-eglcore` / GBM `EINVAL`). The DMA-BUF renderer is disabled there
/// to avoid both failure modes.
fn is_hyprland(compositor: Option<&str>) -> bool {
    matches!(compositor, Some("Hyprland"))
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
    /// This workaround is needed for X11 sessions and for Hyprland Wayland
    /// sessions with NVIDIA drivers.
    DisableWebkitDmabufRenderer,
    /// Disable NVIDIA explicit sync.
    ///
    /// This workaround is needed for non-Hyprland Wayland sessions with NVIDIA
    /// drivers where the dma-buf based `egl-wayland2` library is not in use.
    DisableNvExplicitSync,
}

/// Selects the workaround for a detected session, GPU, compositor, and EGL
/// library state.
///
/// On Hyprland the DMA-BUF renderer is disabled entirely, since it both
/// violates the compositor's acquire-point rule (a protocol error that kills
/// the client) and triggers an NVIDIA EGL/GBM SIGSEGV during rendering. On other
/// Wayland compositors the explicit sync workaround is skipped when the dma-buf
/// based `egl-wayland2` library is in use (NVIDIA driver 560+), since disabling
/// explicit sync would degrade rendering performance; otherwise explicit sync is
/// disabled.
fn workaround_for(
    session: SessionType,
    nvidia_detected: bool,
    hyprland: bool,
    egl_wayland2: bool,
) -> WorkaroundKind {
    if !nvidia_detected {
        return WorkaroundKind::None;
    }
    match session {
        SessionType::Wayland if hyprland => WorkaroundKind::DisableWebkitDmabufRenderer,
        SessionType::Wayland if egl_wayland2 => WorkaroundKind::None,
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
    workaround_for(
        get_session_type(),
        nvidia_detected,
        is_hyprland(get_compositor().as_deref()),
        egl_wayland2_active(),
    )
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

    fn write_manifest(dir: &Path, name: &str, library_path: &str) -> std::io::Result<PathBuf> {
        let path = dir.join(name);
        let content = format!(
            "{{\n  \"file_format_version\": \"1.0.0\",\n  \"ICD\": {{\n    \"library_path\": \"{library_path}\"\n  }}\n}}\n"
        );
        std::fs::write(&path, content)?;
        Ok(path)
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

    mod egl_external_platform {
        use super::*;

        #[test]
        fn classify_wayland_lib_wayland2() {
            assert_eq!(
                classify_wayland_lib("libnvidia-egl-wayland2.so.1"),
                Some(WaylandLib::EglWayland2)
            );
            assert_eq!(
                classify_wayland_lib("/usr/lib/libnvidia-egl-wayland2.so.1.0.1"),
                Some(WaylandLib::EglWayland2)
            );
        }

        #[test]
        fn classify_wayland_lib_old() {
            assert_eq!(
                classify_wayland_lib("libnvidia-egl-wayland.so.1"),
                Some(WaylandLib::EglWayland)
            );
            assert_eq!(
                classify_wayland_lib("/usr/lib/libnvidia-egl-wayland.so.1.1.21"),
                Some(WaylandLib::EglWayland)
            );
        }

        #[test]
        fn classify_wayland_lib_other() {
            assert_eq!(classify_wayland_lib("libnvidia-egl-gbm.so.1"), None);
            assert_eq!(classify_wayland_lib("libnvidia-egl-xcb.so.1"), None);
        }

        #[test]
        fn parses_manifest_library_path() {
            let manifest = r#"{
                "file_format_version": "1.0.0",
                "ICD": {
                    "library_path": "libnvidia-egl-wayland2.so.1"
                }
            }"#;
            assert_eq!(
                library_path_from_manifest(manifest),
                Some("libnvidia-egl-wayland2.so.1")
            );
            assert_eq!(library_path_from_manifest("no library path"), None);
        }

        #[test]
        fn parses_driver_major() {
            let nvr =
                "NVRM version: NVIDIA UNIX Open Kernel Module for x86_64  610.43.03  Release Build";
            assert_eq!(parse_driver_major(nvr), Some(610));
            assert_eq!(
                parse_driver_major("NVRM version: NVIDIA UNIX Open Kernel Module 560.35.03"),
                Some(560)
            );
            assert_eq!(parse_driver_major("no version here"), None);
            assert_eq!(parse_driver_major("x86_64"), None);
        }

        #[test]
        fn json_files_in_dir_sorted() -> std::io::Result<()> {
            let dir = temp_dir("sorted");
            write_manifest(&dir, "20_nvidia_xcb.json", "libnvidia-egl-xcb.so.1")?;
            write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1")?;
            write_manifest(
                &dir,
                "09_nvidia_wayland2.json",
                "libnvidia-egl-wayland2.so.1",
            )?;
            write_manifest(&dir, "not-a-json.txt", "foo")?;
            let files = json_files_in_dir(&dir);
            let names: Vec<String> = files
                .iter()
                .map(|path| path.file_name().unwrap().to_string_lossy().into_owned())
                .collect();
            assert_eq!(
                names,
                vec![
                    "09_nvidia_wayland2.json",
                    "10_nvidia_wayland.json",
                    "20_nvidia_xcb.json",
                ]
            );
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_new_first() -> std::io::Result<()> {
            let dir = temp_dir("new_first");
            let configs = vec![
                write_manifest(
                    &dir,
                    "09_nvidia_wayland2.json",
                    "libnvidia-egl-wayland2.so.1",
                )?,
                write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1")?,
            ];
            assert!(is_egl_wayland2_selected(&configs, Some(610)));
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_only_new() -> std::io::Result<()> {
            let dir = temp_dir("only_new");
            let configs = vec![write_manifest(
                &dir,
                "09_nvidia_wayland2.json",
                "libnvidia-egl-wayland2.so.1",
            )?];
            assert!(is_egl_wayland2_selected(&configs, Some(610)));
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_old_first() -> std::io::Result<()> {
            let dir = temp_dir("old_first");
            let configs = vec![
                write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1")?,
                write_manifest(
                    &dir,
                    "99_nvidia_wayland2.json",
                    "libnvidia-egl-wayland2.so.1",
                )?,
            ];
            assert!(!is_egl_wayland2_selected(&configs, Some(610)));
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_only_old() -> std::io::Result<()> {
            let dir = temp_dir("only_old");
            let configs = vec![write_manifest(
                &dir,
                "10_nvidia_wayland.json",
                "libnvidia-egl-wayland.so.1",
            )?];
            assert!(!is_egl_wayland2_selected(&configs, Some(610)));
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_driver_too_old() -> std::io::Result<()> {
            let dir = temp_dir("driver_too_old");
            let configs = vec![write_manifest(
                &dir,
                "09_nvidia_wayland2.json",
                "libnvidia-egl-wayland2.so.1",
            )?];
            assert!(!is_egl_wayland2_selected(&configs, Some(550)));
            assert!(!is_egl_wayland2_selected(&configs, None));
            Ok(())
        }

        #[test]
        fn egl_wayland2_selected_no_wayland_lib() -> std::io::Result<()> {
            let dir = temp_dir("no_wayland_lib");
            let configs = vec![write_manifest(
                &dir,
                "15_nvidia_gbm.json",
                "libnvidia-egl-gbm.so.1",
            )?];
            assert!(!is_egl_wayland2_selected(&configs, Some(610)));
            assert!(!is_egl_wayland2_selected(&[], Some(610)));
            Ok(())
        }
    }

    mod gpu_detection {
        use super::*;

        #[test]
        fn driver_name_resolves_symlink() -> std::io::Result<()> {
            let dir = temp_dir("driver_name");
            let card = write_fake_card(&dir, "card0", Some("0x10de"), Some("nvidia"), true)?;
            assert_eq!(driver_name(&card), Some("nvidia".to_string()));
            Ok(())
        }

        #[test]
        fn driver_name_missing_symlink() -> std::io::Result<()> {
            let dir = temp_dir("driver_name_missing");
            let card = write_fake_card(&dir, "card0", Some("0x10de"), None, true)?;
            assert_eq!(driver_name(&card), None);
            Ok(())
        }

        #[test]
        fn enumerate_gpus_at_nvidia_primary() -> std::io::Result<()> {
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
        fn enumerate_gpus_at_nouveau_not_nvidia_driver() -> std::io::Result<()> {
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
        fn enumerate_gpus_at_no_cards() {
            let dir = temp_dir("enumerate_empty");
            let devices = enumerate_gpus_at(&dir);
            assert!(devices.is_empty());
            assert!(!nvidia_driver_loaded(&devices));
        }
    }

    mod session_detection {
        use super::*;

        #[test]
        fn xdg_session_type_preferred() {
            assert_eq!(
                session_type_from_env(None, Some("wayland"), None, Some(":0")),
                SessionType::Wayland
            );
            assert_eq!(
                session_type_from_env(None, Some("x11"), Some("wayland-0"), None),
                SessionType::X11
            );
        }

        #[test]
        fn falls_back_to_display_vars() {
            // Simulates a Flatpak sandbox where XDG_SESSION_TYPE isn't
            // propagated, but the socket permission env vars are set.
            assert_eq!(
                session_type_from_env(None, None, Some("wayland-0"), None),
                SessionType::Wayland
            );
            assert_eq!(
                session_type_from_env(None, None, None, Some(":0")),
                SessionType::X11
            );
            assert_eq!(
                session_type_from_env(None, None, None, None),
                SessionType::Unknown
            );
        }

        #[test]
        fn unrecognized_xdg_session_type_falls_back() {
            assert_eq!(
                session_type_from_env(None, Some("tty"), Some("wayland-0"), None),
                SessionType::Wayland
            );
        }

        #[test]
        fn gdk_backend_overrides_xdg_session_type() {
            // GDK_BACKEND=x11 wins even when XDG_SESSION_TYPE says wayland.
            assert_eq!(
                session_type_from_env(Some("x11"), Some("wayland"), Some("wayland-0"), None),
                SessionType::X11
            );
            assert_eq!(
                session_type_from_env(Some("wayland"), Some("x11"), None, Some(":0")),
                SessionType::Wayland
            );
        }

        #[test]
        fn gdk_backend_list_takes_first() {
            // GDK reads a colon/comma separated list and uses the first recognized.
            assert_eq!(
                session_type_from_env(Some("x11,wayland"), Some("wayland"), None, None),
                SessionType::X11
            );
            assert_eq!(
                session_type_from_env(Some("wayland:x11"), Some("x11"), None, None),
                SessionType::Wayland
            );
        }

        #[test]
        fn gdk_backend_unrecognized_falls_through() {
            assert_eq!(
                session_type_from_env(Some("broadway"), Some("wayland"), None, None),
                SessionType::Wayland
            );
            assert_eq!(
                session_type_from_env(Some("broadway"), None, None, None),
                SessionType::Unknown
            );
        }
    }

    mod workaround_selection {
        use super::*;

        #[test]
        fn hyprland_disables_dmabuf_renderer() {
            // Hyprland enforces the acquire-point rule and its NVIDIA EGL/GBM
            // render path SIGSEVs, so the DMABUF renderer is disabled even with
            // egl-wayland2 active (the workaround must not be skipped).
            assert_eq!(
                workaround_for(SessionType::Wayland, true, true, true),
                WorkaroundKind::DisableWebkitDmabufRenderer
            );
        }

        #[test]
        fn egl_wayland2_skips_on_lenient_compositor() {
            // On compositors that tolerate the missing acquire point (e.g. niri),
            // egl-wayland2 works and the workaround is skipped.
            assert_eq!(
                workaround_for(SessionType::Wayland, true, false, true),
                WorkaroundKind::None
            );
        }

        #[test]
        fn wayland_without_egl_wayland2_disables_nv_explicit_sync() {
            assert_eq!(
                workaround_for(SessionType::Wayland, true, false, false),
                WorkaroundKind::DisableNvExplicitSync
            );
        }

        #[test]
        fn x11_disables_dmabuf_renderer() {
            assert_eq!(
                workaround_for(SessionType::X11, true, false, false),
                WorkaroundKind::DisableWebkitDmabufRenderer
            );
        }

        #[test]
        fn unknown_session_is_noop() {
            assert_eq!(
                workaround_for(SessionType::Unknown, true, false, false),
                WorkaroundKind::None
            );
        }

        #[test]
        fn no_nvidia_is_noop() {
            assert_eq!(
                workaround_for(SessionType::Wayland, false, true, true),
                WorkaroundKind::None
            );
            assert_eq!(
                workaround_for(SessionType::X11, false, false, false),
                WorkaroundKind::None
            );
        }

        #[test]
        fn compositor_from_env_prefers_xdg_current_desktop() {
            assert_eq!(
                compositor_from_env(Some("Hyprland"), Some("Hyprland")),
                Some("Hyprland")
            );
            assert_eq!(
                compositor_from_env(Some("niri"), Some("Hyprland")),
                Some("niri")
            );
        }

        #[test]
        fn compositor_from_env_falls_back_to_xdg_session_desktop() {
            assert_eq!(compositor_from_env(None, Some("Hyprland")), Some("Hyprland"));
            assert_eq!(compositor_from_env(None, None), None);
        }

        #[test]
        fn hyprland_compositor_detected() {
            assert!(is_hyprland(Some("Hyprland")));
            assert!(!is_hyprland(Some("niri")));
            assert!(!is_hyprland(None));
        }
    }

    /// Integration test: exercises the full GPU-detection pipeline
    /// (`enumerate_gpus_at` + `nvidia_driver_loaded`) against a fake sysfs
    /// tree that mimics what is actually visible inside a Flatpak sandbox,
    /// i.e. only `/sys/class/drm` is present - there is no
    /// `/sys/module/nvidia` and no `/proc/driver/nvidia/version`. This is a
    /// regression test for the bug where the workaround silently became a
    /// no-op in sandboxes because detection relied on `/sys/module/nvidia`.
    #[test]
    fn needs_workaround_detection_in_simulated_flatpak_sandbox() -> std::io::Result<()> {
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
        let session = session_type_from_env(None, None, Some("wayland-0"), None);
        assert_eq!(session, SessionType::Wayland);

        // With no /proc/driver/nvidia/version and no host EGL config
        // directories reachable, egl-wayland2 can never be detected as
        // active, so the conservative Wayland workaround is selected.
        assert!(!is_egl_wayland2_selected(&[], None));
        assert_eq!(
            workaround_for(
                session,
                primary_gpu_is_nvidia && driver_loaded,
                false,
                false
            ),
            WorkaroundKind::DisableNvExplicitSync
        );
        Ok(())
    }
}
