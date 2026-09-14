#!/usr/bin/env bash
# Regenerates frontend/src/lib/types.ts from the Rust boundary types (ts-rs).
# NOTE: plain `cargo test` / `nextest` also rewrites types.ts but without
# prettier formatting — always use this script, then check formatting.
# CI fails on drift: run this script, then
# git diff --exit-code frontend/src/lib/types.ts
set -euo pipefail

cd "$(dirname "$0")/.."

cargo test -p models --lib ts_export::export_types
pnpm --dir frontend exec prettier --write src/lib/types.ts
