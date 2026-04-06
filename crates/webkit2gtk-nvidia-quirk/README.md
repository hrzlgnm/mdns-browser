# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A crate that provides a workaround for WebKitGTK DMABUF renderer issues on Linux systems with NVIDIA or Nouveau drivers.

## Problem

When running WebKitGTK-based applications (such as Tauri apps) on Linux with NVIDIA or Nouveau drivers, the DMABUF renderer causes rendering issues on X.Org or crashes on Wayland.

See the [full documentation](https://docs.rs/webkit2gtk-nvidia-quirk) for usage and API details.

## Related Issues

- [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
- [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)

## License

MIT
