#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Resolve release artifact SHA256 checksums and expose them as step outputs.
#
# Env:
#   PACKAGE_NAME     AUR package name, e.g. mdns-browser or mdns-browser-bin
#   RELEASE_VERSION  release version without leading v, e.g. 1.13.0
#   TAG_NAME         release tag, e.g. v1.13.0
#   NEEDS_EXE        "true" for mdns-browser-bin (also resolves the
#                    unbundled binary checksum), anything else to skip it
#   GH_TOKEN         token for `gh` (injected by the workflow)
#   GITHUB_OUTPUT    file receiving step outputs (injected by Actions)
#
# Idempotent: outputs are recomputed from scratch on every run; a retry
# appends identical lines and Actions uses the last value for each key.

set -euo pipefail

: "${PACKAGE_NAME:?PACKAGE_NAME must be set}"
: "${RELEASE_VERSION:?RELEASE_VERSION must be set}"
: "${TAG_NAME:?TAG_NAME must be set}"
: "${NEEDS_EXE:?NEEDS_EXE must be set}"
: "${GITHUB_OUTPUT:?GITHUB_OUTPUT must be set}"

if [[ "$PACKAGE_NAME" == "mdns-browser-bin" ]]; then
    assets=$(gh release view "$TAG_NAME" --repo hrzlgnm/mdns-browser --json assets --jq '.assets | map(select(.name | test("\\.sha256$") | not))')
    sum=$(jq -r --arg name "mdns-browser_${RELEASE_VERSION}_amd64.deb" '.[] | select(.name == $name) | .digest' <<<"$assets")
    if [[ -z "$sum" ]]; then
        echo "Error: Failed to get checksum from GitHub API for release $TAG_NAME" >&2
        exit 1
    fi
    if [[ ! "$sum" =~ ^(sha256:)?[0-9a-fA-F]{64}$ ]]; then
        echo "Error: Invalid sha256 checksum from GitHub API for release $TAG_NAME" >&2
        exit 1
    fi
    echo "sha256=$sum" >>"$GITHUB_OUTPUT"
    if [[ "$NEEDS_EXE" == "true" ]]; then
        sum_exe=$(jq -r --arg name "mdns-browser_linux_x64" '.[] | select(.name == $name) | .digest' <<<"$assets")
        if [[ -z "$sum_exe" ]]; then
            echo "Error: Failed to get exe checksum from GitHub API for release $TAG_NAME" >&2
            exit 1
        fi
        if [[ ! "$sum_exe" =~ ^(sha256:)?[0-9a-fA-F]{64}$ ]]; then
            echo "Error: Invalid sha256 exe checksum from GitHub API for release $TAG_NAME" >&2
            exit 1
        fi
        echo "sha256_exe=$sum_exe" >>"$GITHUB_OUTPUT"
    fi
else
    url="https://github.com/hrzlgnm/mdns-browser/releases/download/$TAG_NAME/$TAG_NAME.tar.gz.sha256"
    echo "getting $url"
    sum=$(curl -LfsS --retry 5 --retry-delay 5 --retry-all-errors --connect-timeout 15 "$url" | cut -d' ' -f1)
    if [[ ! "$sum" =~ ^(sha256:)?[0-9a-fA-F]{64}$ ]]; then
        echo "Error: Invalid sha256 checksum from $url" >&2
        exit 1
    fi
    echo "sha256=$sum" >>"$GITHUB_OUTPUT"
fi
