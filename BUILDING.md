# Building Instructions

## Prerequisites

Before you begin, make sure you meet the necessary prerequisites. You can find more details in the official [Tauri Guide](https://tauri.app/start/prerequisites/).

### Rust Additional Targets

You'll need to add an additional Rust target:

- **Target**: `wasm32-unknown-unknown`

To install it, run the following command using `rustup`:

```console
rustup target add wasm32-unknown-unknown
```

### Development Libraries

Make sure the following development libraries are installed:

- `webkit2gtk-devel`
- `libwebkit2gtk41-devel`
- `curl`
- `wget`
- `file`
- `openssl`
- `gtk+3-devel`
- `librsvg-devel`
- `gcc`
- `pkg-config`

You can install these using your package manager. For detailed instructions, refer to the official [Tauri Linux setup guide](https://tauri.app/start/prerequisites/#linux).

#### xbps (Void Linux)

For Void Linux, install these dependencies by running:

```console
sudo xbps-install -Syu
sudo xbps-install -S \
 libwebkit2gtk41-devel \
 curl \
 wget \
 file \
 openssl \
 gtk+3-devel \
 librsvg-devel \
 gcc \
 pkg-config
```

### Rust Crates

You'll also need the following Rust crates:

- `trunk`
- `tauri-cli`

You can install them using `cargo` with the command:

```console
cargo install --locked trunk tauri-cli

```
