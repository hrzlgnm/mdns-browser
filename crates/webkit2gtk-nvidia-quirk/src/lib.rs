// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! # webkit2gtk-nvidia-quirk
//!
//! A crate that provides session-aware workarounds for WebKitGTK rendering issues
//! on Linux systems with NVIDIA or Nouveau drivers.
//!
//! ## Problem
//!
//! When running WebKitGTK-based applications (such as Tauri apps) on Linux
//! with NVIDIA or Nouveau drivers, rendering issues occur that vary by session type:
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
//! This crate detects NVIDIA or Nouveau kernel modules and the session type (X11/Wayland),
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
//! use webkit2gtk_nvidia_quirk::{
//!     should_apply_workaround, set_webkit_disable_dmabuf_renderer,
//!     nv_disable_explicit_sync, WorkaroundKind
//! };
//!
//! let force_disable = std::env::args().any(|arg| arg == "--force-disable-dmabuf");
//! match should_apply_workaround(force_disable) {
//!     WorkaroundKind::DisableWebkitDmabufRenderer => set_webkit_disable_dmabuf_renderer(),
//!     WorkaroundKind::DisableNvExplicitSync => nv_disable_explicit_sync(),
//!     WorkaroundKind::None => {},
//! }
//! ```
//!
//! ## API
//!
//! ### `is_nvidia_detected()`
//!
//! Checks whether NVIDIA or Nouveau kernel modules are loaded.
//!
//! ### `should_apply_workaround(force_disable: bool) -> WorkaroundKind`
//!
//! Determines which workaround should be applied based on NVIDIA detection and session type.
//!
//! - `force_disable` - If `true`, indicates a workaround should be applied regardless of detection
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

enum SessionType {
    Wayland,
    X11,
    None,
}

/// Detects the used session type based upon the XDG_SESSION_TYPE environment variable
fn get_session_type() -> SessionType {
    match std::env::var("XDG_SESSION_TYPE") {
        Ok(session) => match session.as_str() {
            "x11" => SessionType::X11,
            "wayland" => SessionType::Wayland,
            _ => SessionType::None,
        },
        _ => SessionType::None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum WorkaroundKind {
    None,
    DisableWebkitDmabufRenderer,
    DisableNvExplicitSync,
}

/// Checks if a workaround should be applied.
///
/// This function checks if NVIDIA or Nouveau kernel modules are loaded and
/// returns which workaround should be applied.
///
/// # Arguments
///
/// * `force_disable` - If `true`, indicates the workaround should be applied
///   regardless of whether NVIDIA is detected (useful for manual overrides)
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
pub fn should_apply_workaround(force_disable: bool) -> WorkaroundKind {
    let session = get_session_type();
    if force_disable {
        eprintln!("Note: nvidia workaround enabled by force flag.");
    }

    let detected = is_nvidia_detected();
    if !detected && !force_disable {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nvidia_modules_are_correct() {
        assert!(NVIDIA_MODULES.contains(&"nvidia"));
        assert!(NVIDIA_MODULES.contains(&"nouveau"));
    }
}
