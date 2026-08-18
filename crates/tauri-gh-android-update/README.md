# tauri-gh-android-update

[![Crates.io](https://img.shields.io/crates/v/tauri-gh-android-update)](https://crates.io/crates/tauri-gh-android-update)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A Tauri plugin that surfaces new GitHub releases for manual download on
platforms where `tauri-plugin-updater` cannot be used.

`tauri-plugin-updater` does not support Android, where apps must not
self-install (Google Play Store regulations). This plugin fills that gap with
the same `fetch_update` / `install_update` / `can_auto_update` commands the
frontend would otherwise get from the updater plugin, backed by the
`latest.json` update manifest that the Tauri bundler attaches to each GitHub
release. Instead of downloading and installing, `install_update` opens the
release page in the default browser so the user can install manually.

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

The frontend then calls the commands just like with the updater plugin:

- `can_auto_update` - returns whether the app can check for updates
- `fetch_update` - checks the configured GitHub releases for a newer version
- `install_update` - opens the release page for the pending update

## How it works

`fetch_update` fetches the `latest.json` update manifest from the latest
GitHub release of the configured repository, compares its version against the
installed one, and stores a pending update when a newer release exists.
`install_update` then opens the latest release page in the default browser,
where the user can download the new version manually.

The plugin manages its own pending-update state, so no additional `.manage()`
is required.

## License

MIT