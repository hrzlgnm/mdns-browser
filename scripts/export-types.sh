#!/usr/bin/env bash
# Regenerates src/lib/types.ts from the Rust boundary types (ts-rs).
# NOTE: plain `cargo test` / `nextest` also rewrites types.ts but without
# prettier formatting — always use this script, then check formatting.
# CI fails on drift: run this script, then
# git diff --exit-code src/lib/types.ts
set -euo pipefail

cd "$(dirname "$0")/.."

cargo test --manifest-path src-tauri/Cargo.toml -p models --lib ts_export::export_types
pnpm exec prettier --write src/lib/types.ts
