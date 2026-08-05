#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT-0

fn main() {
    #[cfg(desktop)]
    mdns_browser_lib::run();
}
