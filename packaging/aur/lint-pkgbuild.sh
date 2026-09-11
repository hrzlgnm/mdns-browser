#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Generate the PKGBUILD into ~/lint and run the namcap/source checks on it.
# Must run from the repository root (GENERATE_SCRIPT is repo-relative).
#
# Env:
#   GENERATE_SCRIPT  generator script, e.g. ./packaging/aur/generate-mdns-browser-bin.sh
#   RELEASE_VERSION  release version without leading v, e.g. 1.13.0
#   SHA256           .deb checksum (binary package) or tarball checksum file
#                    content (source package)
#   SHA256_EXE       unbundled binary checksum (binary package)
#   TAG_NAME         release tag, e.g. v1.13.0
#   NEEDS_EXE        "true" for mdns-browser-bin, anything else for mdns-browser
#
# Idempotent: the PKGBUILD is regenerated with overwrite on every run.

set -euo pipefail

: "${GENERATE_SCRIPT:?GENERATE_SCRIPT must be set}"
: "${RELEASE_VERSION:?RELEASE_VERSION must be set}"
: "${SHA256:?SHA256 must be set}"
: "${TAG_NAME:?TAG_NAME must be set}"
: "${NEEDS_EXE:?NEEDS_EXE must be set}"

repo_root="$PWD"
mkdir -p "${HOME}/lint"
if [[ "$NEEDS_EXE" == "true" ]]; then
    : "${SHA256_EXE:?SHA256_EXE must be set for binary packages}"
    "$GENERATE_SCRIPT" "$RELEASE_VERSION" "$SHA256" "$SHA256_EXE" "$TAG_NAME" >"${HOME}/lint/PKGBUILD"
else
    "$GENERATE_SCRIPT" "$RELEASE_VERSION" "$SHA256" "$TAG_NAME" >"${HOME}/lint/PKGBUILD"
fi
cd "${HOME}/lint" || exit 1
"$repo_root/packaging/aur/makepkg-lint.sh"
