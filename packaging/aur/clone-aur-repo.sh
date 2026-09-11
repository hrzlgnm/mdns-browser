#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Clone the AUR package repository to ~/aur.
#
# Env:
#   PACKAGE_NAME  AUR package name, e.g. mdns-browser or mdns-browser-bin
#
# Idempotent: any previous checkout is discarded before cloning.

set -euo pipefail

: "${PACKAGE_NAME:?PACKAGE_NAME must be set}"
rm -rf "${HOME}/aur"
git clone --depth=1 "ssh://aur@aur.archlinux.org/${PACKAGE_NAME}.git" "${HOME}/aur"
