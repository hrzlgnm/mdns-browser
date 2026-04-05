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
//! use webkit2gtk_nvidia_quirk::{apply_workaround_if_needed, set_webkit_disable_dmabuf_renderer};
//!
//! // Call early in your application's startup (before spawning threads)
//! // Check if NVIDIA/Mesa is detected
//! let nvidia_detected = apply_workaround_if_needed(false);
//!
//! // If detected, explicitly set the environment variable
//! if nvidia_detected {
//!     set_webkit_disable_dmabuf_renderer();
//! }
//!
//! // Or force-disable via command line argument
//! // set_webkit_disable_dmabuf_renderer();  // Call this to set the env var
//! ```
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

/// Checks if NVIDIA or Nouveau is detected or if forcing is requested.
///
/// This function checks if NVIDIA or Nouveau kernel modules are loaded and
/// returns whether the DMABUF renderer workaround should be applied.
///
/// # Arguments
///
/// * `force_disable` - If `true`, indicates the workaround should be applied
///   regardless of whether NVIDIA is detected (useful for manual overrides)
///
/// # Returns
///
/// `true` if the workaround should be applied (NVIDIA detected or force requested),
/// `false` otherwise.
///
/// # Note
///
/// This function only performs detection. Use [`set_webkit_disable_dmabuf_renderer`]
/// to actually set the environment variable. Call this first, then call the setter
/// if needed - ideally before spawning any threads.
pub fn apply_workaround_if_needed(force_disable: bool) -> bool {
    if force_disable {
        eprintln!("Note: dmabuf renderer disabled by command line arg. Expect degraded renderer performance");
        return true;
    }

    let detected = is_nvidia_detected();
    if detected {
        eprintln!("Note: NVIDIA or Nouveau detected, disabling dmabuf renderer. Expect degraded renderer performance.");
        eprintln!("See https://github.com/tauri-apps/tauri/issues/10702 and https://github.com/tauri-apps/tauri/issues/9304 for more details.");
    }
    detected
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
    std::env::set_var("WEBKIT_DISABLE_DMABUF_RENDERER", "1");
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
