<script lang="ts">
  import { cssClass } from '$lib/css'
  import { setInterfaces } from '$lib/api'
  import { interfaces } from '$lib/store'
  import { pushToast } from '$lib/toast'
  import type { NetworkInterface } from '$lib/types'

  let { disabled = false }: { disabled?: boolean } = $props()

  const layoutClass = cssClass('interfaces-layout')

  // Local editable copy. `lastBackendSelection` records the last enabled
  // set observed from the backend so applied selections are not echoed
  // back as new changes.
  let items = $state<Array<NetworkInterface>>([])
  let lastBackendSelection = $state<Array<string>>([])

  $effect(() => {
    const backend = $interfaces
    const enabled = backend.filter((iface) => iface.enabled).map((iface) => iface.name)
    lastBackendSelection = enabled
    items = backend.map((iface) => ({ ...iface }))
  })

  function labelText(iface: NetworkInterface): string {
    if (iface.addresses.length === 0) return iface.name
    return `${iface.name} (${iface.addresses.join(', ')})`
  }

  async function onToggle(name: string, checked: boolean) {
    items = items.map((iface) => (iface.name === name ? { ...iface, enabled: checked } : iface))
    const enabled = items.filter((iface) => iface.enabled).map((iface) => iface.name)
    if (JSON.stringify(enabled) !== JSON.stringify(lastBackendSelection)) {
      try {
        await setInterfaces(enabled)
      } catch (e) {
        console.error('[mdns-browser] failed to set interfaces:', e)
        pushToast('Failed to set interfaces', String(e))
      }
    }
  }
</script>

<div class={$layoutClass}>
  <details>
    <summary>Network interfaces</summary>
    <div class="interfaces-list">
      {#each items as iface (iface.name)}
        <label>
          <input
            type="checkbox"
            checked={iface.enabled}
            {disabled}
            onchange={(e) => onToggle(iface.name, e.currentTarget.checked)}
          />
          {labelText(iface)}
        </label>
      {/each}
    </div>
  </details>
</div>

<style>
  .interfaces-list {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: flex-start;
    flex-wrap: wrap;
  }
</style>
