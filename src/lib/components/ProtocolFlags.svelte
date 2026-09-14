<script lang="ts">
  import { browseTypes, setProtocolFlags } from '$lib/api'
  import { protocolFlags } from '$lib/store'
  import type { ProtocolFlags } from '$lib/types'

  let { disabled = false }: { disabled?: boolean } = $props()

  // Push user changes to the backend and re-trigger type discovery.
  // `previous` is plain (untracked) state so the effect below only
  // re-runs on store changes; it starts undefined so the initial load
  // never pushes.
  let previous: ProtocolFlags | undefined = undefined

  $effect(() => {
    const flags = $protocolFlags
    if (previous !== undefined && (previous.ipv4 !== flags.ipv4 || previous.ipv6 !== flags.ipv6)) {
      void setProtocolFlags(flags)
      void browseTypes()
    }
    previous = flags
  })

  function sameFlags(a: ProtocolFlags, b: ProtocolFlags): boolean {
    return a.ipv4 === b.ipv4 && a.ipv6 === b.ipv6
  }
</script>

<div class="protocol-flags">
  <label>
    <input
      type="checkbox"
      checked={$protocolFlags.ipv4}
      {disabled}
      onchange={(e) => {
        const next = { ...$protocolFlags, ipv4: e.currentTarget.checked }
        if (!sameFlags(next, $protocolFlags)) protocolFlags.set(next)
      }}
    />
    IPv4
  </label>
  <label>
    <input
      type="checkbox"
      checked={$protocolFlags.ipv6}
      {disabled}
      onchange={(e) => {
        const next = { ...$protocolFlags, ipv6: e.currentTarget.checked }
        if (!sameFlags(next, $protocolFlags)) protocolFlags.set(next)
      }}
    />
    IPv6
  </label>
</div>

<style>
  .protocol-flags {
    display: flex;
    gap: 0.5rem;
    align-items: center;
    justify-content: flex-start;
  }
</style>
