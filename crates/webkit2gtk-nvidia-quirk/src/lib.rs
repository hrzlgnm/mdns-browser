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
//! ### `is_effective_gpu_nvidia() -> bool`
//!
//! Checks whether an NVIDIA GPU is considered as the effective (active) GPU.
//!
//! Returns `true` if an NVIDIA GPU is the primary GPU or if `DRI_PRIME` resolves to
//! an NVIDIA GPU, `false` otherwise.
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

#![cfg(target_os = "linux")]
use std::env;
use udev::Enumerator;

#[derive(Debug)]
struct GpuDevice {
    pci_id: String,
    vendor_id: u16,
    device_id: u16,
    is_primary: bool,
    is_nvidia: bool,
}

fn parse_pci_ids(pci_parent: &udev::Device) -> (u16, u16) {
    let vendor_id = pci_parent
        .property_value("ID_VENDOR_ID")
        .and_then(|v| v.to_str())
        .and_then(|s| u16::from_str_radix(s, 16).ok())
        .unwrap_or(0);

    let device_id = pci_parent
        .property_value("ID_MODEL_ID")
        .and_then(|v| v.to_str())
        .and_then(|s| u16::from_str_radix(s, 16).ok())
        .unwrap_or(0);

    (vendor_id, device_id)
}

fn gpu_cmp(a: &GpuDevice, b: &GpuDevice) -> std::cmp::Ordering {
    match (a.is_primary, b.is_primary) {
        (true, false) => std::cmp::Ordering::Less,
        (false, true) => std::cmp::Ordering::Greater,
        _ => a.pci_id.cmp(&b.pci_id),
    }
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

        let (vendor_id, device_id) = parse_pci_ids(&pci_parent);

        let is_primary = device
            .attribute_value("boot_display")
            .and_then(|v| v.to_str())
            == Some("1");

        let is_nvidia = vendor_id == 0x10de;

        devices.push(GpuDevice {
            pci_id,
            vendor_id,
            device_id,
            is_primary,
            is_nvidia,
        });
    }

    devices.sort_by(gpu_cmp);

    devices
}

#[derive(Debug, PartialEq, Eq)]
enum DriPrime {
    Index(usize),
    PciId(String),
    VendorDevice(u16, u16),
}

/// Parse DRI_PRIME value which may use underscore notation (e.g., `pci-0000_01_02_00`).
///
/// The normalized internal form uses colons and a dot before the function component
/// (e.g., `pci-0000:01:02.00`).
///
/// Supports:
/// - Numeric index (e.g., "0", "1")
/// - PCI ID with underscores (e.g., "pci-0000_01_02_00") normalized to `domain:bus:device.function`
/// - Vendor:Device hex pairs (e.g., "1234:4567")
///
/// The input may end with a trailing `'!'` which is stripped before parsing.
fn parse_dri_prime(prime: impl Into<String>) -> Option<DriPrime> {
    let prime = prime.into();
    // Strip trailing '!' if present
    let prime = prime.strip_suffix('!').unwrap_or(&prime);

    if let Ok(index) = prime.parse::<usize>() {
        return Some(DriPrime::Index(index));
    }
    if prime.starts_with("pci-") {
        // Strip "pci-" prefix and split on underscores
        let value = prime.strip_prefix("pci-").unwrap_or(prime);
        let components: Vec<&str> = value.split('_').collect();

        // Recompose as "domain:bus:device.function" notation
        if components.len() == 4 {
            let normalized = format!(
                "{}:{}:{}.{}",
                components[0], components[1], components[2], components[3]
            );
            return Some(DriPrime::PciId(normalized));
        }
        // Fallback for other formats
        let normalized = value.replace('_', ":");
        return Some(DriPrime::PciId(normalized));
    } else if prime.contains(':') {
        let parts: Vec<&str> = prime.split(':').collect();
        if parts.len() == 2 {
            let vendor_id = u16::from_str_radix(parts[0], 16).ok();
            let device_id = u16::from_str_radix(parts[1], 16).ok();
            if let (Some(vid), Some(did)) = (vendor_id, device_id) {
                return Some(DriPrime::VendorDevice(vid, did));
            }
        }
    }
    None
}

/// Check if the effective (active) GPU is NVIDIA.
///
/// Returns `true` if either the primary GPU (boot_display) is NVIDIA or if `DRI_PRIME`
/// is set and resolves to an NVIDIA device.
///
/// # Resolution Order
///
/// 1. **Primary GPU check**: If the primary GPU (boot_display) is NVIDIA, immediately
///    returns `true`. This takes precedence because the NVIDIA driver does not properly
///    support offloading to other GPUs, meaning `DRI_PRIME` will not work as expected
///    when the NVIDIA GPU is primary.
///
/// 2. **DRI_PRIME resolution**: If `DRI_PRIME` is set, attempts to resolve it in order:
///    - As a numeric index (e.g., "0", "1")
///    - As a PCI ID (e.g., "pci-0000_01_02_00" normalized to "0000:01:02.00")
///    - As a vendor:device hex pair (e.g., "8086:1234")
///
///    Returns the `is_nvidia` status of the resolved device.
///
/// 3. **Fallback**: If neither of the above apply, falls back to checking the first
///    enumerated GPU (index 0).
///
/// # Special Semantics
///
/// Note that this function may return `true` even if the primary (boot_display) GPU
/// is *not* NVIDIA, as long as `DRI_PRIME` resolves to an NVIDIA device. The function
/// name reflects the effective GPU determination but callers should be aware of this behavior.
///
/// # Returns
///
/// `true` if an NVIDIA GPU is considered active per the resolution logic above,
/// `false` otherwise.
pub fn is_effective_gpu_nvidia() -> bool {
    let devices = enumerate_gpus();

    let primary_is_nvidia = devices.iter().any(|d| d.is_primary && d.is_nvidia);
    if primary_is_nvidia {
        return true;
    }

    let dri_prime = env::var("DRI_PRIME").ok();

    if let Some(dri_prime) = dri_prime {
        if let Some(prime) = parse_dri_prime(dri_prime) {
            match prime {
                DriPrime::Index(index) => {
                    return devices.get(index).map(|d| d.is_nvidia).unwrap_or(false);
                }
                DriPrime::PciId(pci_id) => {
                    if let Some(idx) = devices.iter().position(|d| d.pci_id == pci_id) {
                        return devices[idx].is_nvidia;
                    }
                }
                DriPrime::VendorDevice(vendor_id, device_id) => {
                    if let Some(idx) = devices
                        .iter()
                        .position(|d| d.vendor_id == vendor_id && d.device_id == device_id)
                    {
                        return devices[idx].is_nvidia;
                    }
                }
            }
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

    if !is_effective_gpu_nvidia() {
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

    #[test]
    fn test_dri_prime_numeric_index_trailing_bang() {
        assert_eq!(Some(DriPrime::Index(1)), parse_dri_prime("1!"));
    }
    #[test]
    fn test_dri_prime_pci_id_trailing_bang() {
        assert_eq!(
            Some(DriPrime::PciId("0000:01:02.00".into())),
            parse_dri_prime("pci-0000_01_02_00!")
        );
    }
    #[test]
    fn test_dri_prime_vendor_device_trailing_bang() {
        assert_eq!(
            Some(DriPrime::VendorDevice(0x1234, 0x5678)),
            parse_dri_prime("1234:5678!")
        );
    }
    #[test]
    fn test_dri_prime_pci_id() {
        assert_eq!(
            Some(DriPrime::PciId("0000:01:02.01".into())),
            parse_dri_prime("pci-0000_01_02_01")
        );
    }
    #[test]
    fn test_dri_prime_vendor_device() {
        assert_eq!(
            Some(DriPrime::VendorDevice(0x1234, 0x4567)),
            parse_dri_prime("1234:4567")
        );
    }
    #[test]
    fn test_dri_prime_invalid_string() {
        assert_eq!(None, parse_dri_prime("invalid"));
    }

    #[test]
    fn test_sort_primaries_first() {
        let mut devices = [
            GpuDevice {
                pci_id: "0000:01:00.0".to_string(),
                vendor_id: 0x1002,
                device_id: 0x164e,
                is_primary: false,
                is_nvidia: false,
            },
            GpuDevice {
                pci_id: "0000:02:00.0".to_string(),
                vendor_id: 0x10de,
                device_id: 0x2803,
                is_primary: true,
                is_nvidia: true,
            },
            GpuDevice {
                pci_id: "0000:03:00.0".to_string(),
                vendor_id: 0x8086,
                device_id: 0x1234,
                is_primary: false,
                is_nvidia: true,
            },
        ];

        devices.sort_by(gpu_cmp);

        assert!(devices[0].is_primary);
        assert!(!devices[1].is_primary);
        assert!(!devices[2].is_primary);
    }
}
