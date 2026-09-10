// Copyright 2026 hrzlgnm
// SPDX-License-Identifier: MIT

import { invoke } from '@tauri-apps/api/core';

/**
 * Metadata about an available update, as returned by {@link check}.
 *
 * Mirrors the `UpdateMetadata` struct serialized by the plugin's `check`
 * command (`version` is the newer release, `currentVersion` the installed
 * one).
 */
export interface UpdateMetadata {
    version: string;
    currentVersion: string;
}

/**
 * Checks the GitHub releases of the configured repository for a version
 * newer than the installed one.
 *
 * Resolves to the update metadata when a newer release exists — which is
 * also stored as the pending update for {@link downloadAndInstall} — or
 * `null` when the app is up to date.
 */
export async function check(): Promise<UpdateMetadata | null> {
    return invoke('plugin:android-update|check');
}

/**
 * Opens the release page for the pending update in the default browser,
 * where the user can download the new version manually.
 *
 * Rejects when no update is pending (i.e. {@link check} found nothing or
 * has not run).
 */
export async function downloadAndInstall(): Promise<void> {
    return invoke('plugin:android-update|download_and_install');
}
