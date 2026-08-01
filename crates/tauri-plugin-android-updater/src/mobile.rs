// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

use serde::de::DeserializeOwned;
use tauri::plugin::{PluginApi, PluginHandle};
use tauri::Runtime;

#[cfg(target_os = "android")]
const PLUGIN_IDENTIFIER: &str = "com.github.hrzlgnm.mdns_browser.android_updater";

pub fn init<R: Runtime, C: DeserializeOwned>(
    api: PluginApi<R, C>,
) -> crate::Result<Option<PluginHandle<R>>> {
    #[cfg(target_os = "android")]
    {
        let handle = api.register_android_plugin(PLUGIN_IDENTIFIER, "AndroidUpdaterPlugin")?;
        return Ok(Some(handle));
    }
    #[cfg(target_os = "ios")]
    {
        let _ = api;
        Ok(None)
    }
}
