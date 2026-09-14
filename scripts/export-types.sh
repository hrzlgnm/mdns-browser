#!/usr/bin/env bash
# Regenerates frontend/src/lib/types.ts from the Rust boundary types (ts-rs).
# CI fails on drift: git diff --exit-code frontend/src/lib/types.ts
set -euo pipefail

cd "$(dirname "$0")/.."

cargo test -p models --lib ts_export::export_types
pnpm --dir frontend exec prettier --write src/lib/types.ts
