# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Session-aware workarounds for WebKitGTK rendering issues on Linux with NVIDIA driver.

On Wayland the workaround disables NVIDIA explicit sync. WebKitGTK's DMA-BUF
renderer enables the explicit sync protocol on the window surface but does not
always set an acquire point, which compositors enforce strictly and answer with
a protocol error that kills the connection. This is a WebKitGTK bug
([#280210](https://bugs.webkit.org/show_bug.cgi?id=280210)) independent of the
NVIDIA driver version or the Wayland EGL library in use, so the workaround is
applied for every NVIDIA Wayland session.

## Quick Start

```rust,no_run
#[cfg(target_os == "linux")]
{
    use webkit2gtk_nvidia_quirk::{apply_workaround_with_options, ApplyWorkaroundOptions};

    apply_workaround_with_options(ApplyWorkaroundOptions::default());
}
```

See the [full documentation](https://docs.rs/webkit2gtk-nvidia-quirk) for detailed API information.

## License

MIT
