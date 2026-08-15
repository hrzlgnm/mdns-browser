# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Session-aware workarounds for WebKitGTK rendering issues on Linux with NVIDIA driver.

WebKitGTK's DMA-BUF renderer enables the Wayland explicit sync protocol on the
window surface but does not always set an acquire point before committing.
Compositors differ in how strictly they enforce this: Hyprland answers with a
protocol error that kills the connection (see [WebKitGTK Bug
#280210](https://bugs.webkit.org/show_bug.cgi?id=280210)), while others such as
niri tolerate the missing acquire point. On Hyprland the workaround disables
NVIDIA explicit sync. On compositors that tolerate the missing acquire point,
the workaround is skipped when the dma-buf based `egl-wayland2` library is in
use (NVIDIA driver 560 or newer), since disabling explicit sync would degrade
rendering performance.

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
