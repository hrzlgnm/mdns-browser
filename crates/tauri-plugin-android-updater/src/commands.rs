// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;
use tauri::{AppHandle, Manager, Runtime, State};
use tauri_plugin_http::reqwest;

use crate::{AndroidUpdater, Error, Result};

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(60);
const ALLOWED_DOWNLOAD_HOSTS: &[&str] = &["github.com", "objects.githubusercontent.com"];

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DownloadUpdateResult {
    pub version: String,
    pub current_version: String,
    pub apk_path: String,
}

/// Rejects versions containing anything but ASCII alphanumerics, dots, and
/// dashes so they cannot escape `cache_dir` via path separators or traversal.
fn sanitize_version(version: &str) -> Result<&str> {
    if !version
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    {
        return Err(Error::Message(format!("invalid update version: {version}")));
    }
    Ok(version)
}

/// Rejects download URLs that do not use HTTPS on an approved release host.
fn validate_download_url(url: &str) -> Result<()> {
    let parsed = reqwest::Url::parse(url)
        .map_err(|_| Error::Message(format!("invalid download url: {url}")))?;
    if parsed.scheme() != "https" {
        return Err(Error::Message(format!(
            "download url must use https: {url}"
        )));
    }
    let host = parsed.host_str().unwrap_or_default();
    if !ALLOWED_DOWNLOAD_HOSTS.contains(&host) {
        return Err(Error::Message(format!(
            "download url host not allowed: {host}"
        )));
    }
    Ok(())
}

/// Rejects APK paths that do not resolve inside the app cache directory.
fn validate_apk_path<R: Runtime>(app: &AppHandle<R>, path: &str) -> Result<()> {
    let cache_dir = app
        .path()
        .app_cache_dir()
        .map_err(|e| Error::Message(e.to_string()))?
        .canonicalize()
        .map_err(|e| Error::Message(format!("failed to resolve cache dir: {e}")))?;
    let apk_path = PathBuf::from(path)
        .canonicalize()
        .map_err(|e| Error::Message(format!("invalid apk path: {e}")))?;
    if !apk_path.starts_with(&cache_dir) {
        return Err(Error::Message(
            "apk path is outside the app cache directory".to_string(),
        ));
    }
    Ok(())
}

impl<R: Runtime> AndroidUpdater<R> {
    /// Downloads the APK from `url` and triggers its installation.
    pub async fn download_and_install(
        &self,
        app: &AppHandle<R>,
        url: String,
        version: String,
    ) -> Result<DownloadUpdateResult> {
        validate_download_url(&url)?;
        let version = sanitize_version(&version)?;
        let current_version = app.package_info().version.to_string();
        let cache_dir = app
            .path()
            .app_cache_dir()
            .map_err(|e| Error::Message(e.to_string()))?;
        std::fs::create_dir_all(&cache_dir)?;
        let apk_path: PathBuf = cache_dir.join(format!("mdns-browser-{version}.apk"));

        let client = reqwest::Client::builder()
            .timeout(DOWNLOAD_TIMEOUT)
            .build()
            .map_err(|e| Error::Message(format!("failed to build http client: {e}")))?;
        log::info!("Downloading update from {url}");
        let response = client.get(&url).send().await?;
        let bytes = response.bytes().await?;
        std::fs::write(&apk_path, &bytes)?;
        log::info!("Downloaded {} bytes to {apk_path:?}", bytes.len());

        let apk_path = apk_path.to_string_lossy().into_owned();
        self.install_apk(&apk_path).await?;

        Ok(DownloadUpdateResult {
            version: version.to_string(),
            current_version,
            apk_path,
        })
    }

    #[cfg(mobile)]
    pub async fn check_install_permission(&self) -> Result<bool> {
        let handle = self.handle().ok_or(Error::UnsupportedPlatform)?;
        handle
            .run_mobile_plugin_async::<bool>("checkInstallPermission", ())
            .await
            .map_err(Error::from)
    }

    #[cfg(desktop)]
    pub async fn check_install_permission(&self) -> Result<bool> {
        Err(Error::UnsupportedPlatform)
    }

    #[cfg(mobile)]
    pub async fn request_install_permission(&self) -> Result<()> {
        let handle = self.handle().ok_or(Error::UnsupportedPlatform)?;
        handle
            .run_mobile_plugin_async::<()>("requestInstallPermission", ())
            .await
            .map_err(Error::from)
    }

    #[cfg(desktop)]
    pub async fn request_install_permission(&self) -> Result<()> {
        Err(Error::UnsupportedPlatform)
    }

    #[cfg(mobile)]
    async fn install_apk(&self, path: &str) -> Result<()> {
        let handle = self.handle().ok_or(Error::UnsupportedPlatform)?;
        let installed: bool = handle
            .run_mobile_plugin_async::<bool>("installApk", path)
            .await
            .map_err(Error::from)?;
        if installed {
            Ok(())
        } else {
            Err(Error::Message(
                "failed to start APK installation".to_string(),
            ))
        }
    }

    #[cfg(desktop)]
    async fn install_apk(&self, _path: &str) -> Result<()> {
        Err(Error::UnsupportedPlatform)
    }
}

#[tauri::command]
pub async fn check_install_permission<R: Runtime>(
    _app: AppHandle<R>,
    updater: State<'_, AndroidUpdater<R>>,
) -> Result<bool> {
    updater.check_install_permission().await
}

#[tauri::command]
pub async fn request_install_permission<R: Runtime>(
    _app: AppHandle<R>,
    updater: State<'_, AndroidUpdater<R>>,
) -> Result<()> {
    updater.request_install_permission().await
}

#[tauri::command]
pub async fn install_apk<R: Runtime>(
    app: AppHandle<R>,
    path: String,
    updater: State<'_, AndroidUpdater<R>>,
) -> Result<()> {
    validate_apk_path(&app, &path)?;
    updater.install_apk(&path).await
}

#[tauri::command]
pub async fn download_update<R: Runtime>(
    app: AppHandle<R>,
    url: String,
    version: String,
    updater: State<'_, AndroidUpdater<R>>,
) -> Result<DownloadUpdateResult> {
    updater.download_and_install(&app, url, version).await
}
