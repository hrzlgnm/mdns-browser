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
//! ## Wayland with egl-wayland2
//!
//! The Wayland workaround is only needed when the EGLStreams-based `egl-wayland`
//! library is used. NVIDIA drivers 560 and newer ship the dma-buf based
//! `egl-wayland2` library, which implements the Wayland explicit sync protocol
//! correctly and is selected by the EGL loader with higher priority when both
//! libraries are present. On such systems the workaround is skipped automatically,
//! since disabling explicit sync there would degrade rendering performance.
//!
//! Users on older drivers or distributions that do not (yet) package
//! `egl-wayland2` keep the old EGLStreams library and therefore still need the
//! workaround, so it stays enabled for them.
//!
//! ## Detection Method
//!
//! The crate detects the NVIDIA driver by:
//! 1. If the primary/boot GPU (via `boot_display` or `boot_vga` attributes) has vendor ID 0x10de
//! 2. If the proprietary `nvidia` kernel module is loaded (`/sys/module/nvidia` exists)
//!
//! GPU detection uses sysfs exclusively (`/sys/class/drm/`). This provides a simpler and
//! more reliable detection mechanism with no external runtime dependencies.
//!
//! This specifically targets the proprietary NVIDIA driver, not the open-source nouveau driver.
//!
//! Whether `egl-wayland2` is used is detected by mirroring the EGL loader logic:
//! the EGL external platform JSON manifests in `/etc/egl/egl_external_platform.d`
//! and `/usr/share/egl/egl_external_platform.d` (or the directories/files given via
//! `__EGL_EXTERNAL_PLATFORM_CONFIG_DIRS`/`__EGL_EXTERNAL_PLATFORM_CONFIG_FILENAMES`)
//! are checked in load order. If the first manifest referencing an NVIDIA Wayland
//! library points at `libnvidia-egl-wayland2.so` and the loaded driver is version
//! 560 or newer, `egl-wayland2` is considered active and the Wayland workaround is
//! skipped. In all uncertain cases the workaround is still applied.
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

fn enumerate_gpus() -> Vec<GpuDevice> {
    let mut devices = Vec::new();
    let drm_path = PathBuf::from("/sys/class/drm");

    let entries = match std::fs::read_dir(&drm_path) {
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

        devices.push(GpuDevice {
            is_primary,
            is_nvidia,
        });
    }

    devices
}

fn nvidia_driver_loaded() -> bool {
    std::path::Path::new("/sys/module/nvidia").exists()
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

enum SessionType {
    Wayland,
    X11,
    Unknown,
}

/// Detects the used session type based upon the XDG_SESSION_TYPE environment variable
fn get_session_type() -> SessionType {
    match std::env::var("XDG_SESSION_TYPE") {
        Ok(session) => match session.as_str() {
            "x11" => SessionType::X11,
            "wayland" => SessionType::Wayland,
            _ => SessionType::Unknown,
        },
        _ => SessionType::Unknown,
    }
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
    let session = get_session_type();

    if !is_primary_gpu_nvidia() || !nvidia_driver_loaded() {
        return WorkaroundKind::None;
    }
    match session {
        SessionType::Wayland if egl_wayland2_active() => WorkaroundKind::None,
        SessionType::Wayland => WorkaroundKind::DisableNvExplicitSync,
        SessionType::X11 => WorkaroundKind::DisableWebkitDmabufRenderer,
        SessionType::Unknown => WorkaroundKind::None,
    }
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

    fn write_manifest(dir: &Path, name: &str, library_path: &str) -> PathBuf {
        let path = dir.join(name);
        let content = format!(
            "{{\n  \"file_format_version\": \"1.0.0\",\n  \"ICD\": {{\n    \"library_path\": \"{library_path}\"\n  }}\n}}\n"
        );
        std::fs::write(&path, content).unwrap();
        path
    }

    #[test]
    fn test_classify_wayland_lib_wayland2() {
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
    fn test_classify_wayland_lib_old() {
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
    fn test_classify_wayland_lib_other() {
        assert_eq!(classify_wayland_lib("libnvidia-egl-gbm.so.1"), None);
        assert_eq!(classify_wayland_lib("libnvidia-egl-xcb.so.1"), None);
    }

    #[test]
    fn test_library_path_from_manifest() {
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
    fn test_parse_driver_major() {
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
    fn test_json_files_in_dir_sorted() {
        let dir = temp_dir("sorted");
        write_manifest(&dir, "20_nvidia_xcb.json", "libnvidia-egl-xcb.so.1");
        write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1");
        write_manifest(
            &dir,
            "09_nvidia_wayland2.json",
            "libnvidia-egl-wayland2.so.1",
        );
        write_manifest(&dir, "not-a-json.txt", "foo");
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
    }

    #[test]
    fn test_is_egl_wayland2_selected_new_first() {
        let dir = temp_dir("new_first");
        let configs = vec![
            write_manifest(
                &dir,
                "09_nvidia_wayland2.json",
                "libnvidia-egl-wayland2.so.1",
            ),
            write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1"),
        ];
        assert!(is_egl_wayland2_selected(&configs, Some(610)));
    }

    #[test]
    fn test_is_egl_wayland2_selected_only_new() {
        let dir = temp_dir("only_new");
        let configs = vec![write_manifest(
            &dir,
            "09_nvidia_wayland2.json",
            "libnvidia-egl-wayland2.so.1",
        )];
        assert!(is_egl_wayland2_selected(&configs, Some(610)));
    }

    #[test]
    fn test_is_egl_wayland2_selected_old_first() {
        let dir = temp_dir("old_first");
        let configs = vec![
            write_manifest(&dir, "10_nvidia_wayland.json", "libnvidia-egl-wayland.so.1"),
            write_manifest(
                &dir,
                "99_nvidia_wayland2.json",
                "libnvidia-egl-wayland2.so.1",
            ),
        ];
        assert!(!is_egl_wayland2_selected(&configs, Some(610)));
    }

    #[test]
    fn test_is_egl_wayland2_selected_only_old() {
        let dir = temp_dir("only_old");
        let configs = vec![write_manifest(
            &dir,
            "10_nvidia_wayland.json",
            "libnvidia-egl-wayland.so.1",
        )];
        assert!(!is_egl_wayland2_selected(&configs, Some(610)));
    }

    #[test]
    fn test_is_egl_wayland2_selected_driver_too_old() {
        let dir = temp_dir("driver_too_old");
        let configs = vec![write_manifest(
            &dir,
            "09_nvidia_wayland2.json",
            "libnvidia-egl-wayland2.so.1",
        )];
        assert!(!is_egl_wayland2_selected(&configs, Some(550)));
        assert!(!is_egl_wayland2_selected(&configs, None));
    }

    #[test]
    fn test_is_egl_wayland2_selected_no_wayland_lib() {
        let dir = temp_dir("no_wayland_lib");
        let configs = vec![write_manifest(
            &dir,
            "15_nvidia_gbm.json",
            "libnvidia-egl-gbm.so.1",
        )];
        assert!(!is_egl_wayland2_selected(&configs, Some(610)));
        assert!(!is_egl_wayland2_selected(&[], Some(610)));
    }
}
