// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

#![cfg(target_os = "linux")]

use std::path::Path;

const NVIDIA_MODULES: &[&str] = &["nvidia", "nouveau"];

pub fn is_nvidia_detected() -> bool {
    NVIDIA_MODULES.iter().any(|module| {
        let path = format!("/sys/module/{}", module);
        Path::new(&path).exists()
    })
}

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
