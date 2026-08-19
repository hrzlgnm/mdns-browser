// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT-0

//! # tauri-plugin-android-update
//!
//! A Tauri plugin that surfaces new GitHub releases for manual download.
//!
//! [`tauri-plugin-updater`] cannot be used on Android, where apps must not
//! self-install (Google Play Store regulations). This plugin fills that gap
//! with the [`check`] and [`download_and_install`] commands — a custom API
//! modeled on the `tauri-plugin-updater` command names — backed by the
//! `latest.json` update manifest that the release workflow publishes: the
//! Tauri bundler generates it via `createUpdaterArtifacts`, and
//! `tauri-apps/tauri-action` attaches it (signed with the updater signing key
//! configured in the workflow) to each GitHub release. Instead of downloading
//! and installing, `download_and_install` opens the release page in the
//! default browser so the user can install manually.
//!
//! The plugin manages the state the commands need, but does not register the
//! commands itself: they are registered by the consuming app through its own
//! `tauri::generate_handler!` (see [`Builder`]), so the frontend can invoke
//! them under the same unqualified names (`check` / `download_and_install`)
//! as on desktop. Register the plugin on platforms without self-install
//! support (e.g. under `#[cfg(mobile)]`), where desktop apps keep using
//! [`tauri-plugin-updater`].
//!
//! [`tauri-plugin-updater`]: https://docs.rs/tauri-plugin-updater

use std::sync::Mutex;

use tauri::{
    plugin::{Builder as PluginBuilder, TauriPlugin},
    Manager, Runtime,
};

/// Metadata about an available update, as returned by [`check`].
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMetadata {
    /// The version of the available update.
    pub version: String,
    /// The currently installed version.
    pub current_version: String,
}

/// Configures the plugin with the GitHub repository whose releases are
/// checked for updates.
///
/// Either set [`Builder::owner`] and [`Builder::repo`] to derive the URLs
/// from the repository, or provide the URLs explicitly via
/// [`Builder::latest_json_url`] and [`Builder::releases_url`]. Missing
/// configuration is reported when the plugin is set up.
pub struct Builder {
    owner: String,
    repo: String,
    latest_json_url: Option<String>,
    releases_url: Option<String>,
}

impl Default for Builder {
    fn default() -> Self {
        Self::new()
    }
}

impl Builder {
    /// Creates a builder with no URLs configured.
    ///
    /// Configure the repository with [`Builder::owner`] and [`Builder::repo`],
    /// or provide the URLs explicitly with [`Builder::latest_json_url`] and
    /// [`Builder::releases_url`].
    pub fn new() -> Self {
        Self {
            owner: String::new(),
            repo: String::new(),
            latest_json_url: None,
            releases_url: None,
        }
    }

    /// Sets the GitHub owner whose releases are checked for updates.
    ///
    /// Together with [`Builder::repo`] it derives the `latest.json` manifest
    /// URL and the release page that [`download_and_install`] opens.
    pub fn owner(mut self, owner: impl Into<String>) -> Self {
        self.owner = owner.into();
        self
    }

    /// Sets the GitHub repository whose releases are checked for updates.
    ///
    /// Together with [`Builder::owner`] it derives the `latest.json` manifest
    /// URL and the release page that [`download_and_install`] opens.
    pub fn repo(mut self, repo: impl Into<String>) -> Self {
        self.repo = repo.into();
        self
    }

    /// Overrides the URL of the `latest.json` update manifest that [`check`]
    /// reads. Defaults to the manifest attached to the latest release of the
    /// configured repository.
    pub fn latest_json_url(mut self, url: impl Into<String>) -> Self {
        self.latest_json_url = Some(url.into());
        self
    }

    /// Overrides the release page that [`download_and_install`] opens.
    /// Defaults to the latest release of the configured repository.
    pub fn releases_url(mut self, url: impl Into<String>) -> Self {
        self.releases_url = Some(url.into());
        self
    }

    /// Builds the plugin.
    ///
    /// Sets up the state that the [`check`] and [`download_and_install`]
    /// commands rely on. Register it on platforms that cannot self-install
    /// updates (e.g. under `#[cfg(mobile)]`) and add the commands to the
    /// app's own `tauri::generate_handler!` so the frontend can invoke them
    /// unqualified.
    ///
    /// The URLs are resolved when the plugin is set up: an explicitly
    /// configured URL wins, otherwise it is derived from `owner`/`repo`.
    /// Plugin setup fails with an error if neither source is configured.
    pub fn build<R: Runtime>(self) -> TauriPlugin<R> {
        let latest_json_url = self
            .latest_json_url
            .or_else(|| github_latest_json_url(&self.owner, &self.repo));
        let releases_url = self
            .releases_url
            .or_else(|| github_releases_url(&self.owner, &self.repo));

        PluginBuilder::<R>::new("android-update")
            .setup(move |app, _api| {
                let (Some(latest_json_url), Some(releases_url)) = (latest_json_url, releases_url)
                else {
                    let message =
                        "owner and repo, or latest_json_url and releases_url, must be configured"
                            .to_string();
                    log::error!("failed to set up tauri-plugin-android-update: {message}");
                    return Err(message.into());
                };
                app.manage(Config {
                    latest_json_url,
                    releases_url,
                });
                app.manage(PendingUpdateInfo(Mutex::new(None)));
                Ok(())
            })
            .build()
    }
}

/// The URL of the `latest.json` update manifest attached to the latest
/// release of the GitHub repository `owner`/`repo`, if the repository is
/// configured.
fn github_latest_json_url(owner: &str, repo: &str) -> Option<String> {
    repository_configured(owner, repo)
        .then(|| format!("https://github.com/{owner}/{repo}/releases/latest/download/latest.json"))
}

/// The latest release page of the GitHub repository `owner`/`repo`, if the
/// repository is configured.
fn github_releases_url(owner: &str, repo: &str) -> Option<String> {
    repository_configured(owner, repo)
        .then(|| format!("https://github.com/{owner}/{repo}/releases/latest"))
}

fn repository_configured(owner: &str, repo: &str) -> bool {
    !owner.is_empty() && !repo.is_empty()
}

/// The release endpoints the plugin talks to.
///
/// Managed as app state by [`Builder::build`]; the [`check`] and
/// [`download_and_install`] commands take it via `tauri::State`.
pub struct Config {
    latest_json_url: String,
    releases_url: String,
}

/// The pending update stored between [`check`] and [`download_and_install`].
#[derive(Clone)]
struct PendingUpdate {
    version: String,
}

/// App-managed state holding the pending update, if any.
///
/// Managed as app state by [`Builder::build`]; the [`check`] and
/// [`download_and_install`] commands take it via `tauri::State`.
pub struct PendingUpdateInfo(Mutex<Option<PendingUpdate>>);

/// The `latest.json` update manifest published with each release.
#[derive(serde::Deserialize)]
struct LatestJson {
    version: String,
}

/// Compares a fetched release version against the installed version.
///
/// A leading `v` on the fetched version is tolerated, matching how GitHub
/// release tags and the `latest.json` manifest are formatted.
fn compare_versions(fetched: &str, current: &str) -> Result<std::cmp::Ordering, String> {
    let fetched =
        semver::Version::parse(fetched.strip_prefix('v').unwrap_or(fetched)).map_err(|e| {
            log::error!("failed to parse latest release version: {e}");
            format!("failed to parse latest release version: {e}")
        })?;
    let current = semver::Version::parse(current).map_err(|e| {
        log::error!("failed to parse current app version: {e}");
        format!("failed to parse current app version: {e}")
    })?;
    Ok(fetched.cmp(&current))
}

mod commands {
    use super::{
        compare_versions, Config, LatestJson, PendingUpdate, PendingUpdateInfo, UpdateMetadata,
    };
    use tauri::Runtime;
    use tauri_plugin_opener::OpenerExt;

    /// Checks the GitHub releases of the configured repository for a version
    /// newer than the installed one, mirroring the `check` command of
    /// [`tauri-plugin-updater`](https://docs.rs/tauri-plugin-updater).
    ///
    /// Returns the update metadata when a newer release exists and stores it as
    /// the pending update for [`download_and_install`], or `None` when the app is
    /// up to date.
    ///
    /// The consuming app registers this command in its own
    /// `tauri::generate_handler!` under the unqualified `check` name.
    #[tauri::command]
    pub async fn check<R: Runtime>(
        app: tauri::AppHandle<R>,
        config: tauri::State<'_, Config>,
        pending_update: tauri::State<'_, PendingUpdateInfo>,
    ) -> Result<Option<UpdateMetadata>, String> {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(15))
            .build()
            .map_err(|e| {
                log::error!("failed to build http client: {e}");
                format!("failed to build http client: {e}")
            })?;
        let body = client
            .get(&config.latest_json_url)
            .send()
            .await
            .map_err(|e| {
                log::error!("failed to fetch latest release info: {e}");
                format!("failed to fetch latest release info: {e}")
            })?
            .text()
            .await
            .map_err(|e| {
                log::error!("failed to read latest release info: {e}");
                format!("failed to read latest release info: {e}")
            })?;
        let latest_json: LatestJson = serde_json::from_str(&body).map_err(|e| {
            log::error!("failed to parse latest release info: {e}");
            format!("failed to parse latest release info: {e}")
        })?;
        let latest_version = latest_json.version.trim_start_matches('v').to_string();
        let current_version = app.package_info().version.to_string();

        let ordering = compare_versions(&latest_json.version, &current_version)?;

        let mut pending = pending_update.0.lock().map_err(|e| {
            log::error!("failed to lock pending update state: {e}");
            format!("failed to lock pending update state: {e}")
        })?;

        match ordering {
            std::cmp::Ordering::Greater => {
                log::info!("update {latest_version} found");
                *pending = Some(PendingUpdate {
                    version: latest_version.clone(),
                });
                Ok(Some(UpdateMetadata {
                    version: latest_version,
                    current_version,
                }))
            }
            _ => {
                log::info!("app is up to date ({current_version})");
                *pending = None;
                Ok(None)
            }
        }
    }

    /// Opens the release page of the configured repository for the pending
    /// update, where the user can download the new version manually. Named after
    /// the `download_and_install` command of
    /// [`tauri-plugin-updater`](https://docs.rs/tauri-plugin-updater).
    ///
    /// The consuming app registers this command in its own
    /// `tauri::generate_handler!` under the unqualified `download_and_install`
    /// name.
    #[tauri::command]
    pub async fn download_and_install<R: Runtime>(
        app: tauri::AppHandle<R>,
        config: tauri::State<'_, Config>,
        pending_update: tauri::State<'_, PendingUpdateInfo>,
    ) -> Result<(), String> {
        let pending = pending_update
            .0
            .lock()
            .map_err(|e| {
                log::error!("failed to lock pending update state: {e}");
                format!("failed to lock pending update state: {e}")
            })?
            .as_ref()
            .cloned()
            .ok_or_else(|| {
                log::error!("there is no pending update");
                "there is no pending update".to_string()
            })?;

        log::info!(
            "opening releases page for update {}: {}",
            pending.version,
            config.releases_url
        );
        app.opener()
            .open_url(config.releases_url.clone(), None::<String>)
            .map_err(|e| {
                log::error!("failed to open releases page: {e:?}");
                format!("failed to open releases page: {e:?}")
            })?;

        log::info!("releases page opened, user can download the update manually");
        Ok(())
    }
}

pub use commands::{check, download_and_install};

#[cfg(test)]
mod compare_versions_tests {
    use super::compare_versions;
    use std::cmp::Ordering;

    #[test]
    fn test_compare_versions_older_than_installed_is_not_an_update() {
        assert_eq!(compare_versions("1.9.0", "2.0.0"), Ok(Ordering::Less));
    }

    #[test]
    fn test_compare_versions_equal_to_installed_is_not_an_update() {
        assert_eq!(compare_versions("2.0.0", "2.0.0"), Ok(Ordering::Equal));
    }

    #[test]
    fn test_compare_versions_newer_than_installed_is_an_update() {
        assert_eq!(compare_versions("2.0.1", "2.0.0"), Ok(Ordering::Greater));
        assert_eq!(compare_versions("v2.0.1", "2.0.0"), Ok(Ordering::Greater));
    }

    #[test]
    fn test_compare_versions_malformed_is_rejected() {
        assert!(compare_versions("not-a-version", "2.0.0").is_err());
    }

    #[test]
    fn test_compare_versions_double_v_prefix_is_rejected() {
        assert!(compare_versions("vv2.0.1", "2.0.0").is_err());
    }
}

#[cfg(test)]
mod builder_tests {
    use super::{github_latest_json_url, github_releases_url};

    #[test]
    fn test_github_urls_derived_from_owner_and_repo() {
        assert_eq!(
            github_latest_json_url("hrzlgnm", "mdns-browser"),
            Some(
                "https://github.com/hrzlgnm/mdns-browser/releases/latest/download/latest.json"
                    .to_string()
            )
        );
        assert_eq!(
            github_releases_url("hrzlgnm", "mdns-browser"),
            Some("https://github.com/hrzlgnm/mdns-browser/releases/latest".to_string())
        );
    }

    #[test]
    fn test_github_urls_require_owner_and_repo() {
        assert_eq!(github_latest_json_url("", "mdns-browser"), None);
        assert_eq!(github_latest_json_url("hrzlgnm", ""), None);
        assert_eq!(github_latest_json_url("", ""), None);
        assert_eq!(github_releases_url("", "mdns-browser"), None);
    }
}
