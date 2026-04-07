# webkit2gtk-nvidia-quirk

[![Crates.io](https://img.shields.io/crates/v/webkit2gtk-nvidia-quirk)](https://crates.io/crates/webkit2gtk-nvidia-quirk)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

Session-aware workarounds for WebKitGTK rendering issues on Linux with NVIDIA driver.

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
