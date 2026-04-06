// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! # webkit2gtk-nvidia-quirk
//!
//! A crate that provides session-aware workarounds for WebKitGTK rendering issues
//! on Linux systems with NVIDIA drivers.
//!
//! ## Problem
//!
//! When running WebKitGTK-based applications (such as Tauri apps) on Linux
//! with NVIDIA drivers, rendering issues occur that vary by session type:
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
//! This crate detects NVIDIA kernel modules and the session type (X11/Wayland),
//! then allows to apply the appropriate workaround:
//!
//! | Session Type | Workaround | Environment Variable |
//! |-------------|------------|---------------------|
//! | X11 | Disable DMABUF renderer | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
//! | Wayland | Disable NVIDIA explicit sync | `__NV_DISABLE_EXPLICIT_SYNC=1` |
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
//! Checks whether an NVIDIA GPU is considered as primary.
//!
//! Returns `true` if NVIDIA GPU is used as primary, `false` otherwise.
//!
//! ### `should_apply_workaround() -> WorkaroundKind`
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
//! [`should_apply_workaround`] to detect which workaround is needed.
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
//!
//! ## Disclaimer
//!
//! This workaround may not work reliably when using multiple graphics cards
//! (e.g., an integrated GPU provided by the CPU and a discrete GPU).
//! Detection uses udev enumeration of DRM devices and may not accurately
//! reflect the currently active renderer in hybrid setups.

#![cfg(target_os = "linux")]
use std::env;
use udev::Enumerator;

#[derive(Debug)]
struct GpuDevice {
    pci_id: String,
    is_primary: bool,
    is_nvidia: bool,
}

fn enumerate_gpus() -> Vec<GpuDevice> {
    let mut devices = Vec::new();

    let mut enumerator = match Enumerator::new() {
        Ok(e) => e,
        Err(_) => return devices,
    };

    if enumerator.match_subsystem("drm").is_err() {
        return devices;
    }

    let device_iter = match enumerator.scan_devices() {
        Ok(d) => d,
        Err(_) => return devices,
    };

    for device in device_iter {
        let sysname = match device.sysname().to_str() {
            Some(s) => s,
            None => continue,
        };

        if !sysname.starts_with("card") || sysname.contains('-') {
            continue;
        }

        let pci_parent = match device.parent_with_subsystem("pci").ok().flatten() {
            Some(p) => p,
            None => continue,
        };

        let pci_id = pci_parent
            .property_value("PCI_SLOT_NAME")
            .and_then(|v| v.to_str())
            .map(|s| s.to_string())
            .unwrap_or_default();

        if pci_id.is_empty() {
            continue;
        }

        let is_primary = device
            .attribute_value("boot_display")
            .and_then(|v| v.to_str())
            == Some("1");

        let is_nvidia = pci_parent
            .property_value("ID_VENDOR_FROM_DATABASE")
            .and_then(|v| v.to_str())
            .unwrap_or_default()
            .contains("NVIDIA");

        devices.push(GpuDevice {
            pci_id,
            is_primary,
            is_nvidia,
        });
    }

    devices.sort_by(|a, b| match (a.is_primary, b.is_primary) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.pci_id.cmp(&b.pci_id),
    });

    devices
}

/// Check if the active GPU is NVIDIA.
///
/// The active GPU is determined as follows:
/// - If the primary GPU (boot_display) is NVIDIA, return true (DRI_PRIME won't work)
/// - If DRI_PRIME is set, try to match: first as numeric index, then as PCI id
///   (e.g., "pci-0000:01:00.0"), and return that GPU's is_nvidia status
/// - Otherwise fall back to index 0 behavior
///
/// When the primary GPU is NVIDIA, DRI_PRIME does not work as expected
/// because the NVIDIA driver does not properly offload to other GPUs.
///
/// # Returns
///
/// `true` if the primary GPU is an NVIDIA GPU, `false` otherwise.
pub fn is_primary_gpu_nvidia() -> bool {
    let devices = enumerate_gpus();

    let primary_is_nvidia = devices.iter().any(|d| d.is_primary && d.is_nvidia);
    if primary_is_nvidia {
        return true;
    }

    let dri_prime = env::var("DRI_PRIME").ok();

    if let Some(dri_prime) = dri_prime {
        if let Ok(index) = dri_prime.parse::<usize>() {
            return devices.get(index).map(|d| d.is_nvidia).unwrap_or(false);
        }

        let normalized = if dri_prime.starts_with("pci-") {
            dri_prime
                .strip_prefix("pci-")
                .unwrap_or(&dri_prime)
                .to_string()
        } else {
            dri_prime
        };

        if let Some(idx) = devices.iter().position(|d| d.pci_id == normalized) {
            return devices[idx].is_nvidia;
        }
    }

    devices.first().map(|d| d.is_nvidia).unwrap_or(false)
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
/// This function checks if NVIDIA kernel modules are loaded and
/// returns which workaround should be applied.
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
pub fn should_apply_workaround() -> WorkaroundKind {
    let session = get_session_type();

    if !is_primary_gpu_nvidia() {
        return WorkaroundKind::None;
    }
    match session {
        SessionType::Wayland => WorkaroundKind::DisableNvExplicitSync,
        _ => WorkaroundKind::DisableWebkitDmabufRenderer,
    }
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
/// Otherwise, it calls [`should_apply_workaround`] to detect which workaround is needed.
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
        match should_apply_workaround() {
            WorkaroundKind::None => {}
            WorkaroundKind::DisableWebkitDmabufRenderer => set_webkit_disable_dmabuf_renderer(),
            WorkaroundKind::DisableNvExplicitSync => nv_disable_explicit_sync(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    fn set_env_var(key: &str, value: Option<&str>) {
        match value {
            Some(v) => env::set_var(key, v),
            None => env::remove_var(key),
        }
    }

    mod dri_prime_parsing {
        use super::*;

        #[test]
        fn test_dri_prime_numeric_index_zero() {
            set_env_var("DRI_PRIME", Some("0"));
            let _ = is_primary_gpu_nvidia();
            set_env_var("DRI_PRIME", None);
        }

        #[test]
        fn test_dri_prime_numeric_index_one() {
            set_env_var("DRI_PRIME", Some("1"));
            let _ = is_primary_gpu_nvidia();
            set_env_var("DRI_PRIME", None);
        }

        #[test]
        fn test_dri_prime_invalid_string() {
            set_env_var("DRI_PRIME", Some("invalid"));
            let _ = is_primary_gpu_nvidia();
            set_env_var("DRI_PRIME", None);
        }

        #[test]
        fn test_dri_prime_not_set() {
            set_env_var("DRI_PRIME", None);
            let _ = is_primary_gpu_nvidia();
        }
    }

    mod gpu_sorting {
        use super::*;

        #[test]
        fn test_sort_primaries_first() {
            let mut devices = [
                GpuDevice {
                    pci_id: "0000:01:00.0".to_string(),
                    is_primary: false,
                    is_nvidia: false,
                },
                GpuDevice {
                    pci_id: "0000:02:00.0".to_string(),
                    is_primary: true,
                    is_nvidia: true,
                },
                GpuDevice {
                    pci_id: "0000:03:00.0".to_string(),
                    is_primary: false,
                    is_nvidia: true,
                },
            ];

            devices.sort_by(|a, b| match (a.is_primary, b.is_primary) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.pci_id.cmp(&b.pci_id),
            });

            assert!(devices[0].is_primary);
            assert!(!devices[1].is_primary);
            assert!(!devices[2].is_primary);
        }
    }
}
