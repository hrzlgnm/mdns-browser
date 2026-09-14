<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { canAutoUpdate, getVersion, openUrl } from '$lib/api'
  import { desktop } from '$lib/store'
  import { pushToast } from '$lib/toast'
  import { checkUpdate, downloadAndInstall } from '$lib/updater'
  import type { UpdateMetadata } from '$lib/types'

  const GITHUB_BASE_URL = 'https://github.com/hrzlgnm/mdns-browser'
  const SHOW_NO_UPDATE_DURATION_MS = 3000

  let version = $state('')
  let update = $state<UpdateMetadata | null>(null)
  let pendingRid = $state<number | null>(null)
  let canUpdate = $state(false)
  let showNoUpdate = $state(false)
  let timer: ReturnType<typeof setTimeout> | undefined = undefined

  onMount(() => {
    void (async () => {
      try {
        version = await getVersion()
      } catch (e) {
        console.error('[mdns-browser] failed to query version:', e)
      }
      try {
        canUpdate = await canAutoUpdate()
      } catch (e) {
        console.error('[mdns-browser] failed to query can_auto_update:', e)
      }
    })()
  })

  onDestroy(() => {
    clearTimeout(timer)
  })

  function flashNoUpdate() {
    showNoUpdate = true
    clearTimeout(timer)
    timer = setTimeout(() => {
      showNoUpdate = false
    }, SHOW_NO_UPDATE_DURATION_MS)
  }

  async function onCheckUpdate() {
    try {
      const checked = await checkUpdate($desktop)
      if (checked.update === null) flashNoUpdate()
      pendingRid = checked.rid
      update = checked.update
    } catch (e) {
      console.error('[mdns-browser] failed to check for updates:', e)
      pushToast('Update failed', String(e))
    }
  }

  async function onInstallUpdate() {
    try {
      await downloadAndInstall($desktop, pendingRid)
    } catch (e) {
      console.error('[mdns-browser] failed to install update:', e)
      pushToast('Update failed', String(e))
    }
  }
</script>

<div>
  <details>
    <summary>About</summary>
    <div>
      <span>Version {version}</span>
      {#if $desktop}
        <button
          type="button"
          onclick={() => void openUrl(`${GITHUB_BASE_URL}/releases/tag/v${version}`)}
        >
          Release Notes
        </button>
        <button
          type="button"
          onclick={() => void openUrl(`${GITHUB_BASE_URL}/issues/new?template=bug_report.yml`)}
        >
          Report an Issue
        </button>
        <button
          type="button"
          onclick={() =>
            void openUrl(`${GITHUB_BASE_URL}/issues?q=is%3Aopen+is%3Aissue+label%3Abug`)}
        >
          Known Issues
        </button>
        <button type="button" onclick={() => void openUrl(`${GITHUB_BASE_URL}/releases/`)}>
          Releases
        </button>
      {/if}
      {#if showNoUpdate}
        <button type="button">{version} is the latest version</button>
      {:else if canUpdate}
        {#if update}
          <button type="button" onclick={() => void onInstallUpdate()}>
            {$desktop ? 'Download and Install ' : 'Open release page '}{update.version}
          </button>
        {:else}
          <button type="button" onclick={() => void onCheckUpdate()}>Check for updates</button>
        {/if}
      {/if}
    </div>
  </details>
</div>
