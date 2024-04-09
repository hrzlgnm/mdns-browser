# Building instructions

## Prerequisites

### Additional rustc target

- wasm32-unknown-unknown

Can be installed using rustup

```console
rustup add target wasm32-unknown-unknown
```

### Development libraries

- wegkit2gkt-devel curl wget file openssl gtk+3-devel libsvg-devel gcc pkg-config

Can be installed using package-manager

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
