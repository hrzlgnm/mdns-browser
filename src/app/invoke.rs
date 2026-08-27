// Copyright 2024-2025 hrzlgnm
// SPDX-License-Identifier: MIT

use tauri_sys::core::invoke;

pub async fn invoke_no_args(cmd: impl Into<String>) {
    let cmd = cmd.into();
    let _ = invoke::<()>(cmd.as_str(), &()).await;
}
