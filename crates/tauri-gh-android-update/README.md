# tauri-gh-android-update

[![Crates.io](https://img.shields.io/crates/v/tauri-gh-android-update)](https://crates.io/crates/tauri-gh-android-update)
[![License: MIT-0](https://img.shields.io/badge/License-MIT%2D0-blue.svg)](https://opensource.org/license/mit-0/)

A Tauri plugin that surfaces new GitHub releases for manual download on
platforms where `tauri-plugin-updater` cannot be used.

`tauri-plugin-updater` does not support Android, where apps must not
self-install (Google Play Store regulations). This plugin fills that gap with
its own `check` / `download_and_install` commands, modeled on the
`tauri-plugin-updater` command names, backed by the `latest.json` update
manifest that the Tauri bundler attaches to each GitHub release. Instead of
downloading and installing, `download_and_install` opens the release page in
the default browser so the user can install manually.

## Quick Start

Add the plugin to your `src-tauri/Cargo.toml`:

```toml
[dependencies]
tauri-gh-android-update = "0.1"
tauri-plugin-opener = "2"
```

Register it where you do not use `tauri-plugin-updater`, e.g. on mobile:

```rust
#[cfg(mobile)]
.plugin(
    tauri_gh_android_update::Builder::new()
        .owner("owner")
        .repo("repo")
        .build()
)
```

The URLs are derived from the `owner`/`repo` pair. To point the update check
at a custom `latest.json` manifest (or to open a different release page),
provide the URLs explicitly instead:

```rust
tauri_gh_android_update::Builder::new()
    .latest_json_url("https://example.com/app/latest.json")
    .releases_url("https://example.com/app/download")
    .build()
```

## Commands

The plugin registers these commands with `tauri::generate_handler!`. The
command names mirror the `tauri-plugin-updater` plugin's, but the payloads
are this plugin's own — the `tauri-plugin-updater` JavaScript API does not
exist on these platforms, so the frontend invokes them directly:

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