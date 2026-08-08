#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT-0

# Locally test the AUR packaging against the latest GitHub release, mirroring
# the CI `update-aur` job.
#
# Unlike CI (triggered by a git tag push), this script always targets the
# *latest* release. It resolves the release tag/version, fetches the required
# checksums (source tarball sha256 from the `<tag>.tar.gz.sha256` release
# asset; `*-bin` deb+exe digests from the release assets), generates a PKGBUILD
# for each available variant into a fresh temporary directory, lints it, builds
# it, and installs the `*-bin` variant (the source variant is built only).
#
# The script is generic and self-configuring: the repository (owner/name) is
# derived from the local git remote, the package name and the presence of a
# `*-bin` variant are derived from the generate-*.sh helpers in this directory.
# It is identical across the mdns-browser, zux and mdns-tui-browser repos.
#
# Usage: test-aur-local.sh [--no-lint] [--no-install] [--no-build] [--no-cleanup]
#                          [--variant=source|bin|both] [--keep-dir=<path>]
#                          [--repo=owner/name]

set -euo pipefail

# ---------------------------------------------------------------------------
# Configuration
# ---------------------------------------------------------------------------
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
AUR_DIR="$SCRIPT_DIR"
REPO_ROOT="$(cd "$AUR_DIR/../.." && pwd)"

# Script flags.
DO_LINT=true
DO_INSTALL=true
DO_CLEANUP=true
DO_BUILD=true
VARIANT="both"
KEEP_DIR=""
REPO_OVERRIDE=""

# Directories that should be removed on exit (unless --no-cleanup).
CLEANUP_DIRS=()

# Detected per-repo values (filled in below).
OWNER=""
REPO=""
PKG=""
HAS_BIN=false

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
           [--variant=source|bin|both] [--keep-dir=<path>] [--repo=owner/name]

Test the AUR packaging locally against the latest GitHub release.

Options:
  --no-lint        Skip the namcap/verifysource lint step.
  --no-install     Do not run pacman -U for the -bin variant.
  --no-build       Skip the makepkg build step (smoke-test gen + lint only).
  --no-cleanup     Keep the temporary build directories instead of removing them.
  --variant=<v>    Only handle 'source', 'bin', or 'both' (default: both;
                   'bin' is skipped automatically if no -bin generator exists).
  --keep-dir=<p>   Write the variant build dirs under <p> instead of a temp dir.
  --repo=<o/n>     Override the repo (owner/name) instead of auto-detecting
                   from the local git remote.
EOF
}

# Run a command as the current user, escalating to sudo only for the install
# step (which wraps pacman and requires root).
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
        --repo=*)
            REPO_OVERRIDE="${1#*=}"
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
# Resolve repository (owner/name)
# ---------------------------------------------------------------------------
if [[ -n "$REPO_OVERRIDE" ]]; then
    OWNER="${REPO_OVERRIDE%%/*}"
    REPO="${REPO_OVERRIDE#*/}"
    [[ -n "$OWNER" && "$REPO" && "$OWNER" != "$REPO_OVERRIDE" ]] \
        || die "Invalid --repo= value: '$REPO_OVERRIDE' (expected owner/name)"
else
    remote=$(git -C "$REPO_ROOT" remote get-url origin 2>/dev/null) || remote=""
    if [[ -z "$remote" ]]; then
        die "Could not determine the GitHub repo (no 'origin' remote) and no --repo= was given."
    fi
    # Normalize to "github.com/owner/repo" (handles ssh, git@, ssh://, https).
    remote="${remote#git@github.com:}"
    remote="${remote#ssh://git@github.com/}"
    remote="${remote#https://github.com/}"
    remote="${remote#http://github.com/}"
    remote="${remote#github.com:}"
    remote="${remote#github.com/}"
    remote="${remote%.git}"
    OWNER="${remote%%/*}"
    REPO="${remote#*/}"
    [[ -n "$OWNER" && -n "$REPO" && "$REPO" != "$remote" ]] \
        || die "Could not parse owner/repo from remote: '$remote'"
fi

log "Repo: $OWNER/$REPO"

# ---------------------------------------------------------------------------
# Detect package name and bin-variant availability from the generators present
# ---------------------------------------------------------------------------
source_gen="$AUR_DIR/generate-${REPO}.sh"
if [[ ! -f "$source_gen" ]]; then
    # Fallback: infer the package name from the first generate-*.sh that is not
    # a *-bin generator.
    source_gen=""
    for f in "$AUR_DIR"/generate-*.sh; do
        [[ -f "$f" ]] || continue
        b=$(basename "$f" .sh); b=${b#generate-}
        [[ "$b" == *-bin ]] && continue
        source_gen="$f"
        break
    done
fi

if [[ -z "$source_gen" ]]; then
    die "No source generate-*.sh helper found in $AUR_DIR (expected generate-${REPO}.sh or similar)."
fi

# Derive PKG from the resolved source_gen filename.
PKG=$(basename "$source_gen" .sh)
PKG=${PKG#generate-}

# Now recompute bin_gen using the resolved PKG.
bin_gen="$AUR_DIR/generate-${PKG}-bin.sh"
HAS_BIN=false
if [[ -f "$bin_gen" ]]; then
    HAS_BIN=true
fi

log "Package: $PKG (bin variant: $HAS_BIN)"

# ---------------------------------------------------------------------------
# Resolve latest release information
# ---------------------------------------------------------------------------
log "Resolving latest release..."
TAG_NAME=$(gh release view --repo "$OWNER/$REPO" --json tagName --jq '.tagName')
if [[ -z "$TAG_NAME" ]]; then
    die "Could not determine latest release tag (is the repo public and is gh authenticated?)."
fi

# Derive the semantic version: strip a leading "$PKG-v" then a leading "v".
VERSION="$TAG_NAME"
VERSION="${VERSION#"$PKG"-v}"
VERSION="${VERSION#v}"
TAG="$TAG_NAME"

log "Latest release: tag=$TAG version=$VERSION"

# ---------------------------------------------------------------------------
# Checksum resolution
# ---------------------------------------------------------------------------
# Resolvers print their result to stdout and print nothing on failure. They
# deliberately never call `die`, because callers capture their output via
# command substitution (a subshell) where `die`/`exit` would not terminate the
# script. Callers check the result and call `die` in the main shell.
#
# Source variant: checksum of the github archive tarball (published as the
# `<tag>.tar.gz.sha256` release asset).
resolve_source_checksum() {
    local url="https://github.com/$OWNER/$REPO/releases/download/$TAG/$TAG.tar.gz.sha256"
    local sum
    sum=$(curl -LfsS "$url" 2>/dev/null | cut -d' ' -f1) || true
    printf '%s' "$sum"
}

# Binary variant: emits two lines to stdout -- sha256 of the .deb, then sha256
# of the linux x64 executable asset. Empty/short of two lines on failure.
resolve_bin_checksums() {
    local assets sum_deb sum_exe
    assets=$(gh release view --repo "$OWNER/$REPO" --json assets \
        --jq '.assets | map(select(.name | test("\\.sha256$") | not))' 2>/dev/null) || true
    [[ -n "$assets" ]] || return 0
    sum_deb=$(printf '%s' "$assets" \
        | jq -r --arg name "${PKG}_${VERSION}_amd64.deb" \
        '.[] | select(.name == $name) | .digest' 2>/dev/null) || true
    sum_exe=$(printf '%s' "$assets" \
        | jq -r --arg name "${PKG}_linux_x64" \
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
            generate_script="$source_gen"
            extra="source tarball sha256=$source_sha"
            ;;
        bin)
            local bin_sha bin_sha_exe bin_checksums
            bin_checksums=$(resolve_bin_checksums)
            bin_sha=$(printf '%s\n' "$bin_checksums" | sed -n '1p')
            bin_sha_exe=$(printf '%s\n' "$bin_checksums" | sed -n '2p')
            [[ -n "$bin_sha" && -n "$bin_sha_exe" ]] \
                || die "Failed to resolve bin checksums for ${PKG} ${VERSION} (tag $TAG)"
            sha_args=("$VERSION" "$bin_sha" "$bin_sha_exe" "$TAG")
            generate_script="$bin_gen"
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
            "$AUR_DIR/makepkg-lint.sh"
        fi

        if [[ "$DO_BUILD" = true ]]; then
            log "[$kind] Building (makepkg)..."
            makepkg -f
        else
            log "[$kind] Build skipped (--no-build)."
        fi

        local pkg pkg_count
        # Exclude -debug- packages and validate exactly one artifact for install.
        pkg=$(find . -maxdepth 1 -name '*.pkg.tar.*' -type f -printf '%f\n' 2>/dev/null \
            | grep -v -- '-debug-' | sort | tail -n1)
        if [[ -n "$pkg" ]]; then
            log "[$kind] Build artifact: $pkg"
        fi

        if [[ "$kind" = "bin" && "$DO_BUILD" = true && "$DO_INSTALL" = true ]]; then
            pkg_count=$(find . -maxdepth 1 -name '*.pkg.tar.*' -type f -printf '%f\n' 2>/dev/null \
                | grep -v -- '-debug-' | wc -l)
            if [[ "$pkg_count" -eq 0 ]]; then
                die "[$kind] No built package artifact found; cannot install."
            elif [[ "$pkg_count" -gt 1 ]]; then
                die "[$kind] Multiple non-debug package artifacts found; cannot determine which to install."
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
        if [[ "$VARIANT" = "bin" && "$HAS_BIN" = false ]]; then
            die "Variant 'bin' requested but no generate-${PKG}-bin.sh was found."
        fi
        kinds=("$VARIANT")
        ;;
    both)
        if [[ "$HAS_BIN" = true ]]; then
            kinds=(source bin)
        else
            log "No -bin variant detected; building source only."
            kinds=(source)
        fi
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
if [[ "${#CLEANUP_DIRS[@]}" -gt 0 ]]; then
    log "(Temporary build directories are removed on exit; use --no-cleanup to keep them.)"
fi
