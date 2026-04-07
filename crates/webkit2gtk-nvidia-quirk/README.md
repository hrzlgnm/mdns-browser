# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A crate that provides session-aware workarounds for WebKitGTK rendering issues on Linux systems with the proprietary NVIDIA driver.

## Problem

When running WebKitGTK-based applications (such as Tauri apps) on Linux with the proprietary NVIDIA driver, rendering issues occur that vary by session type:

- **X11**: The DMABUF renderer causes visual artifacts and rendering issues
- **Wayland**: The DMABUF renderer may crash or hang

## Solution

This crate detects the proprietary NVIDIA driver and the session type (X11/Wayland), then applies the appropriate workaround:

| Session Type | Workaround | Environment Variable |
|-------------|------------|---------------------|
| X11 | Disable DMABUF renderer | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
| Wayland | Disable NVIDIA explicit sync | `__NV_DISABLE_EXPLICIT_SYNC=1` |

## Detection Method

The crate detects the proprietary NVIDIA driver by checking:

1. If the primary GPU (`boot_display` attribute) is NVIDIA (vendor ID 0x10de)
2. If the proprietary `nvidia` kernel module is loaded (`/sys/module/nvidia` exists) AND any NVIDIA GPU is present in the system

This specifically targets the proprietary NVIDIA driver, not the open-source nouveau driver.

## Usage

```rust,no_run
use webkit2gtk_nvidia_quirk::{apply_workaround_with_options, ApplyWorkaroundOptions};

apply_workaround_with_options(ApplyWorkaroundOptions::default()
    .force_disable_dmabuf(true)); // Force disable dmabuf renderer
```

One can also use the following API instead:

```rust,no_run
use webkit2gtk_nvidia_quirk::{needs_workaround, set_webkit_disable_dmabuf_renderer, nv_disable_explicit_sync, WorkaroundKind};

match needs_workaround() {
    WorkaroundKind::DisableWebkitDmabufRenderer => set_webkit_disable_dmabuf_renderer(),
    WorkaroundKind::DisableNvExplicitSync => nv_disable_explicit_sync(),
    WorkaroundKind::None => {},
}
```

See the [full documentation](https://docs.rs/webkit2gtk-nvidia-quirk) for detailed API information.

## Related Issues

- [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
- [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)
- [WebKitGTK Bug #280210](https://bugs.webkit.org/show_bug.cgi?id=280210)

## License

MIT

## Disclaimer

This workaround specifically detects the proprietary NVIDIA driver (the `nvidia` kernel module), not the open-source nouveau driver. It will not apply workarounds when using nouveau or when the NVIDIA GPU is not the primary GPU in the system.

Detection is based on udev enumeration of DRM devices, using the `boot_display` attribute to identify the primary GPU.
