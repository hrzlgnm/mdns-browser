#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Fail unless RELEASE_VERSION is strictly higher than the pkgver recorded
# in the ~/aur checkout. A missing PKGBUILD (new package) passes.
#
# Env:
#   RELEASE_VERSION  release version without leading v, e.g. 1.13.0
#
# Idempotent: read-only check.

set -euo pipefail

: "${RELEASE_VERSION:?RELEASE_VERSION must be set}"

cd "${HOME}/aur" || exit 1
if [[ -f PKGBUILD ]]; then
    # shellcheck disable=SC1091,SC2154 # PKGBUILD is generated into ~/aur at runtime
    current_version=$(source PKGBUILD && echo "$pkgver")
    if [[ "$(printf '%s\n%s' "$current_version" "$RELEASE_VERSION" | sort -V | head -n1)" != "$current_version" ]] || [[ "$current_version" == "$RELEASE_VERSION" ]]; then
        echo "New version ($RELEASE_VERSION) is not higher than the current version ($current_version). Exiting." >&2
        exit 1
    fi
fi
