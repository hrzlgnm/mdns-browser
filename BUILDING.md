# Building Instructions

## Prerequisites

Before you begin, make sure you meet the necessary prerequisites. You can find more details in the official [Tauri Guide](https://tauri.app/start/prerequisites/).

### Node.js and pnpm

The frontend is a SvelteKit app in `frontend/`. You'll need:

- **Node.js**: version 24 or newer
- **pnpm**: version 12 (see the `packageManager` field in `frontend/package.json`)

Install frontend dependencies with:

```console
pnpm --dir frontend install --frozen-lockfile
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

#### pacman (Arch Linux)

For Arch Linux, install these dependencies by running:

```console
sudo pacman -Syu
sudo pacman -S --needed \
 webkit2gtk-4.1 \
 curl \
 wget \
 file \
 openssl \
 gtk3 \
 librsvg \
 gcc \
 pkgconf
```

### Rust Crates

You'll also need the following Rust crate:

- `tauri-cli`

You can install it using `cargo` with the command:

```console
cargo install --locked tauri-cli
```
