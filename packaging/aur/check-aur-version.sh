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
    pkgver_line=$(grep -E '^pkgver=' PKGBUILD | head -n1 || true)
    if [[ -z "$pkgver_line" ]]; then
        echo "Error: no literal pkgver= assignment found in PKGBUILD." >&2
        exit 1
    fi
    current_version="${pkgver_line#pkgver=}"
    if [[ "$current_version" == "'*'" ]]; then
        current_version="${current_version#\'}"
        current_version="${current_version%\'}"
    elif [[ "$current_version" == '"*"' ]]; then
        current_version="${current_version#\"}"
        current_version="${current_version%\"}"
    fi
    if [[ ! "$current_version" =~ ^[A-Za-z0-9._+]+$ ]]; then
        echo "Error: PKGBUILD pkgver is not a plain literal version: $pkgver_line" >&2
        exit 1
    fi
    if [[ "$(printf '%s\n%s' "$current_version" "$RELEASE_VERSION" | sort -V | head -n1)" != "$current_version" ]] || [[ "$current_version" == "$RELEASE_VERSION" ]]; then
        echo "New version ($RELEASE_VERSION) is not higher than the current version ($current_version). Exiting." >&2
        exit 1
    fi
fi
