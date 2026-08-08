#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT-0

# Locally test the AUR packaging for both mdns-browser (source) and
# mdns-browser-bin (binary) variants, mirroring the CI `update-aur` job.
#
# Unlike CI (which is triggered by a git tag push), this script always targets
# the *latest* GitHub release. It fetches the release tag/version, resolves the
# required checksums, generates a PKGBUILD for each variant into a fresh
# temporary directory, lints it, builds it, and installs the -bin variant.
#
# Usage: test-aur-local.sh [--no-lint] [--no-install] [--no-build] [--no-cleanup]
#                          [--variant=source|bin|both] [--keep-dir=<path>]

set -euo pipefail

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "$SCRIPT_DIR/../.." && pwd)"

OWNER="hrzlgnm"
REPO="mdns-browser"

# Script flags.
DO_LINT=true
DO_INSTALL=true
DO_CLEANUP=true
DO_BUILD=true
VARIANT="both"
KEEP_DIR=""

# Directories that should be removed on exit (unless --no-cleanup).
CLEANUP_DIRS=()

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------
log() {
    printf '\033[1;34m[aur-test]\033[0m %s\n' "$*"
}

warn() {
    printf '\033[1;33m[aur-test] WARNING:\033[0m %s\n' "$*" >&2
}

die() {
    printf '\033[1;31m[aur-test] ERROR:\033[0m %s\n' "$*" >&2
    exit 1
}

cleanup() {
    for d in "${CLEANUP_DIRS[@]}"; do
        if [[ -n "$d" && -d "$d" ]]; then
            rm -rf "$d"
        fi
    done
}
trap cleanup EXIT

usage() {
    cat <<EOF
Usage: $0 [--no-lint] [--no-install] [--no-build] [--no-cleanup]
           [--variant=source|bin|both] [--keep-dir=<path>]

Test the AUR packaging locally against the latest GitHub release.

Options:
  --no-lint        Skip the namcap/verifysource lint step.
  --no-install     Do not run makepkg --install for the -bin variant.
  --no-build       Skip the makepkg build step (useful to smoke-test
                   generation + lint + source verification only).
  --no-cleanup     Keep the temporary build directories instead of removing them.
  --variant=<v>    Only handle 'source', 'bin', or 'both' (default: both).
  --keep-dir=<p>   Write the variant build dirs under <p> instead of a temp dir.
EOF
}

# Run a command as the current user, escalating to sudo only when needed and
# only for the install step (which wraps pacman and requires root).
as_sudo() {
    if [[ $EUID -eq 0 ]]; then
        "$@"
    elif sudo -n true 2>/dev/null; then
        sudo -E "$@"
    else
        return 255
    fi
}

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------
while [[ $# -gt 0 ]]; do
    case "$1" in
        --no-lint)
            DO_LINT=false
            shift
            ;;
        --no-install)
            DO_INSTALL=false
            shift
            ;;
        --no-build)
            DO_BUILD=false
            shift
            ;;
        --no-cleanup)
            DO_CLEANUP=false
            shift
            ;;
        --variant=*)
            VARIANT="${1#*=}"
            case "$VARIANT" in
                source|bin|both) ;;
                *) die "Invalid variant: $VARIANT (expected source|bin|both)" ;;
            esac
            shift
            ;;
        --keep-dir=*)
            KEEP_DIR="${1#*=}"
            shift
            ;;
        -h|--help)
            usage
            exit 0
            ;;
        *)
            die "Unknown option: $1"
            ;;
    esac
done

# ---------------------------------------------------------------------------
# Resolve latest release information
# ---------------------------------------------------------------------------
log "Resolving latest release for $OWNER/$REPO..."
TAG_NAME=$(gh release view --repo "$OWNER/$REPO" --json tagName --jq '.tagName')
if [[ -z "$TAG_NAME" ]]; then
    die "Could not determine latest release tag (is the repo public and does gh have access?)."
fi

# Mirror the tag-name stripping done by .github/actions/latest-release-info.
SEMVER="${TAG_NAME#mdns-browser-v}"
if [[ "$SEMVER" = "$TAG_NAME" ]]; then
    SEMVER="${TAG_NAME#v}"
fi
VERSION="$SEMVER"
TAG="$TAG_NAME"

log "Latest release: tag=$TAG version=$VERSION"

# ---------------------------------------------------------------------------
# Checksum resolution
# ---------------------------------------------------------------------------
# These resolvers print their result to stdout and print nothing on failure.
# They deliberately never call `die`, because callers capture their output via
# command substitution (a subshell) where `die`/`exit` would not terminate the
# script. Callers are responsible for checking the result and calling `die`.
#
# Source variant: checksum of the github archive tarball.
resolve_source_checksum() {
    local url="https://github.com/$OWNER/$REPO/releases/download/$TAG/$TAG.tar.gz.sha256"
    local sum
    sum=$(curl -LfsS "$url" 2>/dev/null | cut -d' ' -f1) || true
    printf '%s' "$sum"
}

# Binary variant: emits two lines to stdout: sha256 of the .deb, then sha256
# of the linux x64 executable asset. Empty/short of two lines on failure.
resolve_bin_checksums() {
    local assets sum_deb sum_exe
    assets=$(gh release view --repo "$OWNER/$REPO" --json assets \
        --jq '.assets | map(select(.name | test("\\.sha256$") | not))' 2>/dev/null) || true
    [[ -n "$assets" ]] || return 0
    sum_deb=$(printf '%s' "$assets" \
        | jq -r --arg name "mdns-browser_${VERSION}_amd64.deb" \
        '.[] | select(.name == $name) | .digest' 2>/dev/null) || true
    sum_exe=$(printf '%s' "$assets" \
        | jq -r --arg name "mdns-browser_linux_x64" \
        '.[] | select(.name == $name) | .digest' 2>/dev/null) || true
    if [[ "$sum_deb" != "null" && "$sum_exe" != "null" && -n "$sum_deb" && -n "$sum_exe" ]]; then
        printf '%s\n%s' "$sum_deb" "$sum_exe"
    fi
}

# ---------------------------------------------------------------------------
# Per-variant build driver
# ---------------------------------------------------------------------------
# run_variant <kind> <workdir>
run_variant() {
    local kind="$1" workdir="$2"
    local generate_script sha_args extra

    case "$kind" in
        source)
            local source_sha
            source_sha=$(resolve_source_checksum)
            [[ -n "$source_sha" ]] \
                || die "Failed to fetch source checksum from https://github.com/$OWNER/$REPO/releases/download/$TAG/$TAG.tar.gz.sha256"
            sha_args=("$VERSION" "$source_sha" "$TAG")
            generate_script="$REPO_ROOT/packaging/aur/generate-mdns-browser.sh"
            extra="source tarball sha256=$source_sha"
            ;;
        bin)
            local bin_sha bin_sha_exe bin_checksums
            bin_checksums=$(resolve_bin_checksums)
            bin_sha=$(printf '%s\n' "$bin_checksums" | sed -n '1p')
            bin_sha_exe=$(printf '%s\n' "$bin_checksums" | sed -n '2p')
            [[ -n "$bin_sha" && -n "$bin_sha_exe" ]] \
                || die "Failed to resolve bin checksums for version $VERSION (tag $TAG)"
            sha_args=("$VERSION" "$bin_sha" "$bin_sha_exe" "$TAG")
            generate_script="$REPO_ROOT/packaging/aur/generate-mdns-browser-bin.sh"
            extra="deb=$bin_sha exe=$bin_sha_exe"
            ;;
        *)
            die "Unknown variant kind: $kind"
            ;;
    esac

    log "===== [$kind] Generating PKGBUILD in $workdir ====="
    log "[$kind] $extra"

    "$generate_script" "${sha_args[@]}" > "$workdir/PKGBUILD"

    (
        cd "$workdir"

        if [[ "$DO_LINT" = true ]]; then
            log "[$kind] Linting (namcap + makepkg --verifysource -o)..."
            "$REPO_ROOT/packaging/aur/makepkg-lint.sh"
        fi

        if [[ "$DO_BUILD" = true ]]; then
            log "[$kind] Building (makepkg)..."
            makepkg -f
        else
            log "[$kind] Build skipped (--no-build)."
        fi

        local pkg
        pkg=$(find . -maxdepth 1 -name '*.pkg.tar.*' -type f -printf '%f\n' 2>/dev/null | sort | tail -n1)
        if [[ -n "$pkg" ]]; then
            log "[$kind] Build artifact: $pkg"
        fi

        if [[ "$kind" = "bin" && "$DO_BUILD" = true && "$DO_INSTALL" = true ]]; then
            if [[ -z "$pkg" ]]; then
                die "[$kind] No built package artifact found; cannot install."
            fi
            log "[$kind] Installing $pkg (pacman -U)..."
            if as_sudo pacman -U --noconfirm "$workdir/$pkg"; then
                log "[$kind] Installed successfully."
            else
                die "[$kind] Failed to install $pkg (requires root). Install manually: sudo pacman -U '$workdir/$pkg'"
            fi
        fi
    )
}

# ---------------------------------------------------------------------------
# Prepare working directories
# ---------------------------------------------------------------------------
mk_workdir() {
    local kind="$1"
    if [[ -n "$KEEP_DIR" ]]; then
        mkdir -p "$KEEP_DIR"
        mktemp -d -p "$KEEP_DIR" "${kind}.XXXXXX"
    else
        mktemp -d
    fi
}

# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
log "Repo root: $REPO_ROOT"

case "$VARIANT" in
    source|bin)
        kinds=("$VARIANT")
        ;;
    both)
        kinds=(source bin)
        ;;
    *)
        die "Invalid variant selection: $VARIANT"
        ;;
esac

for k in "${kinds[@]}"; do
    dir=$(mk_workdir "$k")
    if [[ -z "$KEEP_DIR" && "$DO_CLEANUP" = true ]]; then
        CLEANUP_DIRS+=("$dir")
    fi
    run_variant "$k" "$dir"
done

log "All requested variants processed."
if [[ "$DO_CLEANUP" = true ]]; then
    log "(Temporary build directories will be removed on exit unless built with --no-cleanup.)"
fi
