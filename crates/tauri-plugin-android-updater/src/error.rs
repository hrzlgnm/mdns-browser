// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

use serde::{ser::Serializer, Serialize};
use tauri_plugin_http::reqwest;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Network(#[from] reqwest::Error),
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[cfg(mobile)]
    #[error(transparent)]
    PluginInvoke(#[from] tauri::plugin::mobile::PluginInvokeError),
    #[error("tauri error: {0}")]
    Tauri(#[from] tauri::Error),
    #[error("auto update is not supported on this platform")]
    UnsupportedPlatform,
    #[error("{0}")]
    Message(String),
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type Result<T> = std::result::Result<T, Error>;
