import { check as checkDesktopUpdate, Update } from '@tauri-apps/plugin-updater'
import {
  check as checkAndroidUpdate,
  downloadAndInstall as installAndroidUpdate,
} from 'tauri-plugin-android-update-api'
import { restartApp } from './api'
import type { UpdateMetadata } from './types'

// Pending update handle. Desktop carries the `Update` resource (which owns
// the backend `rid`); Android carries plain metadata since the pending
// update is tracked Rust-side by the plugin.
export type PendingUpdate = Update | UpdateMetadata | null

export async function checkUpdate(desktop: boolean): Promise<PendingUpdate> {
  if (desktop) return checkDesktopUpdate()
  return checkAndroidUpdate()
}

export async function downloadAndInstall(desktop: boolean, update: PendingUpdate): Promise<void> {
  if (update === null) throw new Error('there is no pending update')
  if (desktop) {
    if (!(update instanceof Update)) throw new Error('there is no pending update')
    try {
      await update.downloadAndInstall((event) => {
        console.debug('[mdns-browser] update download event:', event)
      })
    } finally {
      await update.close()
    }
    await restartApp()
    return
  }
  await installAndroidUpdate()
}

export async function closeUpdate(update: PendingUpdate): Promise<void> {
  if (update instanceof Update) await update.close()
}
