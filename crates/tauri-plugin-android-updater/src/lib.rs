// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! Tauri plugin that downloads an APK and triggers its installation on Android.
//! On non-Android platforms the plugin is a no-op and its commands return
//! [`Error::UnsupportedPlatform`].

use tauri::{
    plugin::{Builder, PluginHandle, TauriPlugin},
    Manager, Runtime,
};

mod commands;
mod error;
#[cfg(mobile)]
mod mobile;

pub use commands::DownloadUpdateResult;
pub use error::{Error, Result};

/// Managed state holding the handle to the native Android plugin.
pub struct AndroidUpdater<R: Runtime>(Option<PluginHandle<R>>);

impl<R: Runtime> AndroidUpdater<R> {
    #[cfg(mobile)]
    fn handle(&self) -> Option<&PluginHandle<R>> {
        self.0.as_ref()
    }
}

/// Initializes the plugin.
pub fn init<R: Runtime>() -> TauriPlugin<R> {
    Builder::new("android-updater")
        .invoke_handler(tauri::generate_handler![
            commands::check_install_permission,
            commands::download_update,
            commands::install_apk,
            commands::request_install_permission,
        ])
        .setup(|app, _api| {
            #[cfg(mobile)]
            let handle = mobile::init(_api)?;
            #[cfg(not(mobile))]
            let handle: Option<PluginHandle<R>> = None;
            app.manage(AndroidUpdater(handle));
            Ok(())
        })
        .build()
}
