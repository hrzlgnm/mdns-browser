# Building Instructions

## Prerequisites

Before you begin, make sure you meet the necessary prerequisites. You can find more details in the official [Tauri Guide](https://tauri.app/start/prerequisites/).

### Node.js and pnpm

The frontend is a SvelteKit app at the repository root. You'll need:

- **Node.js**: version 24 or newer
- **pnpm**: version 12 (see the `packageManager` field in `package.json`)

Install frontend dependencies with:

```console
pnpm install --frozen-lockfile
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

### Rust Toolchain

The Tauri CLI is provided via pnpm (`pnpm tauri`), so no `cargo install`
is needed for a regular build. For auditable release builds, route `cargo`
through `scripts/cargo` (which delegates to `cargo-auditable`):

```console
cargo install --locked cargo-auditable
PATH="$PWD/scripts:$PATH" pnpm tauri build
```
