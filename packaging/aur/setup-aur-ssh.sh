#!/usr/bin/env bash
# Copyright 2026 hrzlgnm
# SPDX-License-Identifier: MIT
#
# Install the AUR SSH deploy key and pin the aur.archlinux.org host keys
# to the Arch-published fingerprints.
#
# Env:
#   AUR_DEPLOY_KEY  SSH private key for the aur.archlinux.org service user
#
# Idempotent: the key file is overwritten, the ssh config stanza is
# rewritten to exactly one copy, and previously recorded AUR host keys are
# replaced, so a retry neither duplicates entries nor fails.

set -euo pipefail

: "${AUR_DEPLOY_KEY:?AUR_DEPLOY_KEY must be set}"

mkdir -p "${HOME}/.ssh"
chmod 700 "${HOME}/.ssh"
printf '%s\n' "$AUR_DEPLOY_KEY" >"${HOME}/.ssh/aur"
chmod 600 "${HOME}/.ssh/aur"

config="${HOME}/.ssh/config"
touch "$config"
chmod 600 "$config"
tmp_config=$(mktemp "${HOME}/.ssh/config.XXXXXX")
scan_tmp=$(mktemp "${HOME}/.ssh/aur-known-hosts.XXXXXX")
key_patterns=$(mktemp "${HOME}/.ssh/aur-key-patterns.XXXXXX")
tmp_known_hosts=$(mktemp "${HOME}/.ssh/known-hosts.XXXXXX")
trap 'rm -f "$tmp_config" "$scan_tmp" "$key_patterns" "$tmp_known_hosts"' EXIT
awk '/^Host aur\.archlinux\.org([[:space:]]|$)/ { skip = 1; next } /^Host / { skip = 0 } !skip' "$config" >"$tmp_config"
cat >>"$tmp_config" <<'EOF'
Host aur.archlinux.org
  IdentityFile ~/.ssh/aur
  User aur
EOF
mv "$tmp_config" "$config"

# Arch-published AUR SSH host key fingerprints, see
# https://archlinux.org/news/aur-migration-new-ssh-hostkeys/
aur_ssh_fps="SHA256:RFzBCUItH9LZS0cKB5UE6ceAYhBD5C8GeOBip8Z11+4 SHA256:uTa/0PndEgPZTf76e1DFqXKJEXKsn7m9ivhLQtzGOCI SHA256:5s5cIyReIfNNVGRFdDbe3hdYiI5OelHGpw2rOUud3Q8"
ssh-keyscan -H aur.archlinux.org >"$scan_tmp"
if [[ ! -s "$scan_tmp" ]]; then
    echo "::error::ssh-keyscan returned no host keys for aur.archlinux.org" >&2
    exit 1
fi
verified=0
while read -r host type key; do
    case "$host" in '#'*) continue ;; "") continue ;; esac
    fp=$(printf '%s %s' "$type" "$key" | ssh-keygen -lf /dev/stdin -E sha256 | awk '{print $2}')
    case " $aur_ssh_fps " in
        *" $fp "*) ;;
        *)
            echo "::error::AUR host key $type fingerprint $fp not in Arch-published set, refusing" >&2
            exit 1
            ;;
    esac
    echo "Verified AUR host key $type $fp"
    verified=$((verified + 1))
done <"$scan_tmp"
if [[ "$verified" -eq 0 ]]; then
    echo "::error::ssh-keyscan returned no verifiable host keys for aur.archlinux.org" >&2
    exit 1
fi
touch "${HOME}/.ssh/known_hosts"
ssh-keygen -R aur.archlinux.org -f "${HOME}/.ssh/known_hosts" >/dev/null 2>&1 || true
# Drop previously recorded lines carrying the same key material. Key bodies
# are stable across scans even though -H salts differ, so a retry keeps
# exactly one copy of each key instead of accumulating stale hashed lines.
awk '{print $3}' "$scan_tmp" >"$key_patterns"
grep -v -F -f "$key_patterns" "${HOME}/.ssh/known_hosts" >"$tmp_known_hosts" || true
mv "$tmp_known_hosts" "${HOME}/.ssh/known_hosts"
cat "$scan_tmp" >>"${HOME}/.ssh/known_hosts"
