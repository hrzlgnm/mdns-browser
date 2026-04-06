# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

A crate that provides session-aware workarounds for WebKitGTK rendering issues on Linux systems with NVIDIA or Nouveau drivers.

## Problem

When running WebKitGTK-based applications (such as Tauri apps) on Linux with NVIDIA or Nouveau drivers, rendering issues occur that vary by session type:

- **X11**: The DMABUF renderer causes visual artifacts and rendering issues
- **Wayland**: The DMABUF renderer may crash or hang

## Solution

This crate detects NVIDIA or Nouveau kernel modules and the session type (X11/Wayland), then applies the appropriate workaround:

| Session Type | Workaround | Environment Variable |
|-------------|------------|---------------------|
| X11 | Disable DMABUF renderer | `WEBKIT_DISABLE_DMABUF_RENDERER=1` |
| Wayland | Disable NVIDIA explicit sync | `__NV_DISABLE_EXPLICIT_SYNC=1` |

## Usage

```rust,no_run
use webkit2gtk_nvidia_quirk::{should_apply_workaround, set_webkit_disable_dmabuf_renderer, nv_disable_explicit_sync, WokraroundKind};

let force_disable = std::env::args().any(|arg| arg == "--force-disable-dmabuf");
match should_apply_workaround(force_disable) {
    WokraroundKind::DisableWebkitDmabufRenderer => set_webkit_disable_dmabuf_renderer(),
    WokraroundKind::DisableNvExplicitSync => nv_disable_explicit_sync(),
    WokraroundKind::None => {},
}
```

See the [full documentation](https://docs.rs/webkit2gtk-nvidia-quirk) for detailed API information.

## Related Issues

- [tauri-apps/tauri#10702](https://github.com/tauri-apps/tauri/issues/10702)
- [tauri-apps/tauri#9304](https://github.com/tauri-apps/tauri/issues/9304)
- [WebKitGTK Bug #280210](https://bugs.webkit.org/show_bug.cgi?id=280210)

## License

MIT
