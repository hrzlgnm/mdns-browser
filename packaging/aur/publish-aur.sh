#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Regenerate ~/aur/PKGBUILD, verify it builds, and push the update.
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
# Idempotent: regeneration overwrites and the commit happens only on actual
# changes, while the push always runs, so a retry after a committed-but-
# unpushed run just pushes instead of failing on an empty commit.

set -euo pipefail

: "${GENERATE_SCRIPT:?GENERATE_SCRIPT must be set}"
: "${RELEASE_VERSION:?RELEASE_VERSION must be set}"
: "${SHA256:?SHA256 must be set}"
: "${TAG_NAME:?TAG_NAME must be set}"
: "${NEEDS_EXE:?NEEDS_EXE must be set}"

if [[ "$NEEDS_EXE" == "true" ]]; then
    : "${SHA256_EXE:?SHA256_EXE must be set for binary packages}"
    "$GENERATE_SCRIPT" "$RELEASE_VERSION" "$SHA256" "$SHA256_EXE" "$TAG_NAME" >"${HOME}/aur/PKGBUILD"
else
    "$GENERATE_SCRIPT" "$RELEASE_VERSION" "$SHA256" "$TAG_NAME" >"${HOME}/aur/PKGBUILD"
fi
cd "${HOME}/aur" || exit 1
if [[ -n "$(git status --porcelain)" ]]; then
    makepkg --printsrcinfo >.SRCINFO
    makepkg
    makepkg --install --noconfirm
    git config user.name "hrzlgnm"
    git config user.email "hrzlgnm@users.noreply.github.com"
    git add PKGBUILD .SRCINFO
    git commit -m "New upstream release $RELEASE_VERSION"
else
    echo "No changes (already committed on previous retry or up to date)"
fi
# Always push: on retry after commit-but-push-failed the worktree is clean,
# so exiting early would skip the push. A failed push stays a failed attempt
# and is retried.
git push origin master
