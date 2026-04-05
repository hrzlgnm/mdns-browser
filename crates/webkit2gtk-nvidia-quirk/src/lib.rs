// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! # webkit2gtk-nvidia-quirk
//!
//! A crate that provides a workaround for WebKitGTK DMABUF renderer issues
//! on Linux systems with NVIDIA or Mesa drivers.
//!
//! ## Problem
//!
//! When running WebKitGTK-based applications (such as Tauri apps) on Linux
//! with NVIDIA drivers, the DMABUF renderer can cause degraded performance
//! or rendering issues. This is a known upstream issue in WebKitGTK and Tauri.
//!
//! Related upstream issues:
//! - [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
//! - [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)
//!
//! ## Solution
//!
//! This crate detects NVIDIA or Nouveau kernel modules and automatically
//! disables the DMABUF renderer by setting the `WEBKIT_DISABLE_DMABUF_RENDERER`
//! environment variable.
//!
//! ## Usage
//!
//! ```rust,no_run
//! use webkit2gtk_nvidia_quirk::apply_workaround_if_needed;
//!
//! // Call early in your application's startup
//! // force_disable = true will always disable DMABUF regardless of driver
//! let nvidia_detected = apply_workaround_if_needed(false);
//!
//! if nvidia_detected {
//!     println!("NVIDIA/Mesa detected - DMABUF renderer disabled");
//! }
//! ```
//!
//! ## Features
//!
//! - `tauri`: Enables Tauri-specific integration (optional)
//!
//! ## Platform Support
//!
//! This crate is Linux-only and provides no functionality on other platforms.

#![cfg(target_os = "linux")]

use std::path::Path;

const NVIDIA_MODULES: &[&str] = &["nvidia", "nouveau"];

/// Detects whether NVIDIA or Nouveau kernel modules are loaded.
///
/// This function checks for the presence of `nvidia` or `nouveau` modules
/// in `/sys/module/`.
///
/// # Returns
///
/// `true` if either NVIDIA or Nouveau module is detected, `false` otherwise.
pub fn is_nvidia_detected() -> bool {
    NVIDIA_MODULES.iter().any(|module| {
        let path = format!("/sys/module/{}", module);
        Path::new(&path).exists()
    })
}

/// Applies the DMABUF renderer workaround if needed.
///
/// This function checks if NVIDIA or Nouveau is detected and sets the
/// `WEBKIT_DISABLE_DMABUF_RENDERER` environment variable to disable the
/// DMABUF renderer, which can cause performance issues with NVIDIA drivers.
///
/// # Arguments
///
/// * `force_disable` - If `true`, always disables the DMABUF renderer
///   regardless of whether NVIDIA is detected (useful for manual overrides)
///
/// # Returns
///
/// `true` if the workaround was applied (NVIDIA detected or force disabled),
/// `false` otherwise.
///
/// # Note
///
/// This function prints informational messages to stderr when the
/// workaround is applied.
pub fn apply_workaround_if_needed(force_disable: bool) -> bool {
    if force_disable {
        eprintln!("Note: dmabuf renderer disabled by command line arg. Expect degraded renderer performance");
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
        return true;
    }

    let detected = is_nvidia_detected();
    if detected {
        eprintln!("Note: NVIDIA or Nouveau detected, disabling dmabuf renderer. Expect degraded renderer performance.");
        eprintln!("See https://github.com/tauri-apps/tauri/issues/10702 and https://github.com/tauri-apps/tauri/issues/9304 for more details.");
        std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
    }
    detected
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvidia_modules_are_correct() {
        assert!(NVIDIA_MODULES.contains(&"nvidia"));
        assert!(NVIDIA_MODULES.contains(&"nouveau"));
    }
}
