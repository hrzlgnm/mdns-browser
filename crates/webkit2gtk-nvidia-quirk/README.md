# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Session-aware workarounds for WebKitGTK rendering issues on Linux with NVIDIA driver.

WebKitGTK's DMA-BUF renderer enables the Wayland explicit sync protocol on the
window surface but does not always set an acquire point before committing.
Compositors differ in how strictly they enforce this: Hyprland answers with a
protocol error that kills the connection (see
[WebKitGTK Bug #280210](https://bugs.webkit.org/show_bug.cgi?id=280210)),
while others such as niri tolerate the missing acquire point. On Hyprland the
workaround disables the WebKit DMA-BUF renderer, which also avoids a separate
NVIDIA EGL/GBM SIGSEGV that occurs during rendering there. On compositors that
tolerate the missing acquire point, the workaround is skipped when the
dma-buf-based `egl-wayland2` library is in use (NVIDIA driver 560 or newer),
since disabling explicit sync would degrade rendering performance.

The session type is detected from `GDK_BACKEND` first (the backend
GDK/WebKitGTK actually selects - a comma-separated list where the first
recognized entry of `x11`/`wayland` wins; unrecognized entries are ignored),
then `XDG_SESSION_TYPE`, then `WAYLAND_DISPLAY`/`DISPLAY`.

## Quick Start

```rust,no_run
#[cfg(target_os == "linux")]
{
    use webkit2gtk_nvidia_quirk::{apply_workaround_with_options, ApplyWorkaroundOptions};

    apply_workaround_with_options(ApplyWorkaroundOptions::default());
}
```

See the [full documentation](https://docs.rs/webkit2gtk-nvidia-quirk) for detailed API information.

## Verbosity

Diagnostic output is controlled by the `WEBKIT2GTK_NVIDIA_QUIRK_VERBOSE`
environment variable, independently of any `verbose` argument passed to the API:

| Value | Output |
|-------|--------|
| unset / `0` / `false` / `off` | none |
| `1` / `true` / `yes` / `on` | per-workaround diagnostic note |
| `debug` / `trace` / `verbose` / `2` | note **and** a detection summary |

The detection summary (session type, whether the primary GPU is NVIDIA, whether
the NVIDIA driver is loaded, the detected compositor, Hyprland, `egl-wayland2`
state, the chosen workaround, and which workaround environment variables are
set) is printed to stderr and is intended for troubleshooting why a particular
workaround was applied. It is env-var controlled only; the builder API cannot
enable it.

## License

MIT
