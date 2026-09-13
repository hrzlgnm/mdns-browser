import { Channel, invoke } from '@tauri-apps/api/core'
import type { UpdateMetadata } from './types'

// Mirrors the update flow in the former Leptos `src/app/about.rs`.
// Desktop goes through the `plugin:updater` commands directly; the pending
// update is identified by the resource id (`rid`) that `download_and_install`
// must be given together with a progress channel.

interface UpdaterMetadata {
  rid: number
  version: string
  currentVersion: string
}

type DownloadEvent =
  | { event: 'Started'; data: { contentLength?: number } }
  | { event: 'Progress'; data: { chunkLength: number } }
  | { event: 'Finished' }

export interface CheckedUpdate {
  update: UpdateMetadata | null
  rid: number | null
}

export async function checkUpdate(desktop: boolean): Promise<CheckedUpdate> {
  if (desktop) {
    const metadata = await invoke<UpdaterMetadata | null>('plugin:updater|check')
    if (metadata === null) return { update: null, rid: null }
    return {
      update: { version: metadata.version, currentVersion: metadata.currentVersion },
      rid: metadata.rid,
    }
  }
  const metadata = await invoke<UpdateMetadata | null>('plugin:android-update|check')
  return { update: metadata, rid: null }
}

export async function downloadAndInstall(desktop: boolean, rid: number | null): Promise<void> {
  if (desktop) {
    if (rid === null) throw new Error('there is no pending update')
    const onEvent = new Channel<DownloadEvent>()
    onEvent.onmessage = (event) => {
      console.info('[mdns-browser] update download event:', event)
    }
    await invoke('plugin:updater|download_and_install', { rid, onEvent })
    await invoke('restart')
    return
  }
  await invoke('plugin:android-update|download_and_install')
}
