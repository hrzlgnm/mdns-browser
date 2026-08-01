// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

const COMMANDS: &[&str] = &[
    "check_install_permission",
    "download_update",
    "install_apk",
    "request_install_permission",
];

fn main() {
    tauri_plugin::Builder::new(COMMANDS)
        .android_path("android")
        .build();
}
