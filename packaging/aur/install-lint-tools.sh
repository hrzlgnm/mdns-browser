#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Install pinned shellcheck and shfmt binaries into ~/.local/bin.
# Pinned releases keep lint results reproducible across runner image updates.
# Bump the versions together with their checksums below.
#
# Idempotent: archives are re-downloaded and binaries overwritten on every
# run, so a retry leaves the same final state.

set -euo pipefail

shellcheck_version="v0.11.0"
shfmt_version="v3.14.1"

arch="$(uname -m)"
case "$arch" in
    x86_64 | amd64)
        shellcheck_asset="shellcheck-${shellcheck_version}.linux.x86_64.tar.xz"
        shellcheck_sha256="8c3be12b05d5c177a04c29e3c78ce89ac86f1595681cab149b65b97c4e227198"
        shfmt_asset="shfmt_${shfmt_version}_linux_amd64"
        shfmt_sha256="76e77641faa025814b77f153b29796b8e6fa2fca03e0c76a691608b86c7ea7bf"
        ;;
    aarch64 | arm64)
        shellcheck_asset="shellcheck-${shellcheck_version}.linux.aarch64.tar.xz"
        shellcheck_sha256="12b331c1d2db6b9eb13cfca64306b1b157a86eb69db83023e261eaa7e7c14588"
        shfmt_asset="shfmt_${shfmt_version}_linux_arm64"
        shfmt_sha256="5f2db09dae91fca848f7adbdd014632e921a383863a2ad7e0450ad3aba0c6489"
        ;;
    *)
        echo "Error: unsupported architecture '$arch'" >&2
        exit 1
        ;;
esac

bindir="${HOME}/.local/bin"
workdir="$(mktemp -d)"
trap 'rm -rf "$workdir"' EXIT
mkdir -p "$bindir"

curl_flags=(-LfsS --retry 5 --retry-delay 5 --retry-all-errors --connect-timeout 15)
curl "${curl_flags[@]}" -o "$workdir/$shellcheck_asset" "https://github.com/koalaman/shellcheck/releases/download/${shellcheck_version}/${shellcheck_asset}"
curl "${curl_flags[@]}" -o "$workdir/$shfmt_asset" "https://github.com/mvdan/sh/releases/download/${shfmt_version}/${shfmt_asset}"

printf '%s  %s\n' "$shellcheck_sha256" "$workdir/$shellcheck_asset" | sha256sum -c -
printf '%s  %s\n' "$shfmt_sha256" "$workdir/$shfmt_asset" | sha256sum -c -

tar -xJf "$workdir/$shellcheck_asset" -C "$workdir"
install -m 755 "$workdir/shellcheck-${shellcheck_version}/shellcheck" "$bindir/shellcheck"
install -m 755 "$workdir/$shfmt_asset" "$bindir/shfmt"

if [[ -n "${GITHUB_PATH:-}" ]]; then
    printf '%s\n' "$bindir" >>"$GITHUB_PATH"
fi

"$bindir/shellcheck" --version
"$bindir/shfmt" --version
