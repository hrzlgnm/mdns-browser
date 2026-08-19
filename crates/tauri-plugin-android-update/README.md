# tauri-plugin-android-update

[![Crates.io](https://img.shields.io/crates/v/tauri-plugin-android-update)](https://crates.io/crates/tauri-plugin-android-update)
[![License: MIT-0](https://img.shields.io/badge/License-MIT%2D0-blue.svg)](https://opensource.org/license/mit-0/)

A Tauri plugin that surfaces new GitHub releases for manual download on
platforms where `tauri-plugin-updater` cannot be used.

`tauri-plugin-updater` does not support Android, where apps must not
self-install (Google Play Store regulations). This plugin fills that gap with
its own `check` / `download_and_install` commands, modeled on the
`tauri-plugin-updater` command names, backed by the `latest.json` update
manifest that the release workflow publishes: the Tauri bundler generates it
via `createUpdaterArtifacts`, and `tauri-apps/tauri-action` attaches it
(signed with the updater signing key configured in the workflow) to each
GitHub release. Instead of downloading and installing, `download_and_install`
opens the release page in the default browser so the user can install
manually.

## Quick Start

Add the plugin to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-plugin-android-update = "0.1"
tauri-plugin-opener = "2"
```

Register it where you do not use `tauri-plugin-updater`, e.g. on mobile, and
add its commands to your app's `invoke_handler` under their unqualified names:

```rust
#[cfg(mobile)]
.plugin(
    tauri_plugin_android_update::Builder::new()
        .owner("owner")
        .repo("repo")
        .build()
)
.invoke_handler(tauri::generate_handler![
    tauri_plugin_android_update::check,
    tauri_plugin_android_update::download_and_install,
    // ...
])
```

The URLs are derived from the `owner`/`repo` pair. To point the update check
at a custom `latest.json` manifest (or to open a different release page),
provide the URLs explicitly instead:

```rust
tauri_plugin_android_update::Builder::new()
    .latest_json_url("https://example.com/app/latest.json")
    .releases_url("https://example.com/app/download")
    .build()
```

## Commands

The plugin manages the state its commands rely on but leaves the registration
to the app, so the commands are invoked under the same unqualified names on
every platform. The command names mirror the `tauri-plugin-updater` plugin's,
but the payloads are this plugin's own — the `tauri-plugin-updater` JavaScript
API does not exist on these platforms, so the frontend invokes them directly:

- `check` — fetches the `latest.json` update manifest from the latest release,
  compares its version against the installed one, and resolves to the update
  metadata (`{ version, currentVersion }`) when a newer release exists, or
  `null` when the app is up to date. A newer release is also stored as the
  pending update for `download_and_install`.
- `download_and_install` — opens the release page for the pending update in
  the default browser, where the user can download the new version manually.
  Errors when no update is pending (i.e. `check` found nothing or has not
  run).

## How it works

The plugin manages its own pending-update state, so no additional `.manage()`
is required.

## License

MIT-0