<script lang="ts">
  import { cssClass } from '#lib/css.js'
  import { daemonStatus, metrics } from '#lib/store.js'
  import type { Membership, SocketState } from 'tauri-plugin-mdns-api'

  const layoutClass = cssClass('diagnostics-layout')

  // The running variant of the daemon status, or null when shut down or
  // not yet known.
  const running = $derived($daemonStatus?.status === 'running' ? $daemonStatus : null)

  const sorted = $derived(
    Object.entries($metrics)
      .filter(([, value]) => value !== 0)
      .sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)),
  )

  function socketLabel(socket: SocketState): string {
    if (socket.bound) return 'bound'
    return socket.bind_error === null ? 'not bound' : `not bound (${socket.bind_error})`
  }

  function membershipLabel(family: string, membership: Membership | null): string {
    if (membership === null) return `${family} none`
    if (membership.joined) return `${family} joined`
    return membership.join_error === null
      ? `${family} not joined`
      : `${family} not joined (${membership.join_error})`
  }
</script>

<div class={$layoutClass}>
  <details>
    <summary>Diagnostics</summary>
    {#if running !== null}
      <div class="daemon-status">
        <span>Daemon: running</span>
        <span>IPv4 socket: {socketLabel(running.ipv4_socket)}</span>
        <span>IPv6 socket: {socketLabel(running.ipv6_socket)}</span>
        {#each running.enabled_interfaces as iface (iface.index)}
          <span>
            {iface.name}: {membershipLabel('IPv4', iface.ipv4)}, {membershipLabel(
              'IPv6',
              iface.ipv6,
            )}
          </span>
        {/each}
      </div>
    {:else if $daemonStatus !== null}
      <div class="daemon-status">
        <span>Daemon: {$daemonStatus.status}</span>
      </div>
    {/if}
    <details>
      <summary>mDNS-SD-metrics</summary>
      <div class="diagnostics-grid">
        {#each sorted as [name, value] (name)}
          <div class="diagnostic-item">
            <i>{name}</i>
            <span class="diagnostic-badge">{value}</span>
          </div>
        {/each}
      </div>
    </details>
  </details>
</div>

<style>
  .daemon-status {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    align-items: flex-start;
    padding-bottom: 0.5rem;
  }
  .diagnostic-item i {
    margin-right: 0.25em;
  }
  .diagnostic-badge {
    display: inline-block;
    padding: 0.1em 0.6em;
    border-radius: 1em;
    font-size: 1.1em;
  }
</style>
