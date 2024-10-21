# Building instructions

## Prerequisites

More information is available in [Tauri-Prerequisites](https://tauri.app/start/prerequisites/)

### Additional targets for rust

- wasm32-unknown-unknown

Installation using rustup

```console
rustup target add wasm32-unknown-unknown
```

### Development libraries

- wegkit2gkt-devel curl wget file openssl gtk+3-devel libsvg-devel gcc pkg-config

Installation using package-manager

See also official [tauri-instructions](https://tauri.app/start/prerequisites/#linux)

#### xbps

```console
sudo xbps-install -Syu
sudo xbps-install -S \
    webkit2gtk-devel \
    curl \
    wget \
    file \
    openssl \
    gtk+3-devel \
    librsvg-devel \
    gcc \
    pkg-config
```

### Crates

- trunk
- tauri-cli

Those can be installed using cargo.

```console
cargo install --locked trunk tauri-cli
```
