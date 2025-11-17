#!/usr/bin/env bash
set -e

# Run namcap and capture output + exit status
echo ">>> Running namcap..."
namcap_output=$(namcap PKGBUILD 2>&1)
namcap_status=$?

echo "$namcap_output"

if [[ $namcap_status -ne 0 ]]; then
    echo ">>> Namcap crashed (exit code $namcap_status). Aborting."
    exit $namcap_status
fi

if echo "$namcap_output" | grep -E -q "^(W|E): "; then
    echo ">>> Namcap warnings/errors detected. Aborting."
    exit 1
fi

echo ">>> Namcap clean."

echo ">>> Verifying sources with makepkg  ..."
makepkg --verifysource -o
