# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A crate that provides a workaround for WebKitGTK DMABUF renderer issues on Linux systems with NVIDIA or Nouveau drivers.

## Problem

When running WebKitGTK-based applications (such as Tauri apps) on Linux with NVIDIA or Nouveau drivers, the DMABUF renderer causes rendering issues on X.Org or crashes on Wayland. This is a known upstream issue in WebKitGTK and Tauri.

Related upstream issues:
- [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
- [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)

## Solution

This crate detects NVIDIA or Nouveau kernel modules and provides functions to disable the DMABUF renderer by setting the `WEBKIT_DISABLE_DMABUF_RENDERER` environment variable.

## Usage

```rust
use webkit2gtk_nvidia_quirk::{should_disable_dmabuf_renderer, set_webkit_disable_dmabuf_renderer};

// Call early in your application's startup (before spawning threads).

// This example shows how to disable the DMABUF renderer if NVIDIA/Nouveau is
// detected, or if a command-line flag `--force-disable-dmabuf` is present.
let force_disable = std::env::args().any(|arg| arg == "--force-disable-dmabuf");
if should_disable_dmabuf_renderer(force_disable) {
    set_webkit_disable_dmabuf_renderer();
}
```

## API

### `is_nvidia_detected()`

Checks whether NVIDIA or Nouveau kernel modules are loaded.

```rust
let detected = is_nvidia_detected();
```

### `should_disable_dmabuf_renderer(force_disable: bool)`

Checks if the DMABUF renderer workaround should be applied.

- `force_disable` - If `true`, indicates the workaround should be applied regardless of detection

Returns `true` if the workaround should be applied.

### `set_webkit_disable_dmabuf_renderer()`

Sets the `WEBKIT_DISABLE_DMABUF_RENDERER` environment variable. Must be called before spawning threads.

## Platform Support

This crate is Linux-only and provides no functionality on other platforms.

## License

MIT
