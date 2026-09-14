<script lang="ts">
  import { cssClass } from '$lib/css'
  import { metrics } from '$lib/store'

  const layoutClass = cssClass('metrics-layout')

  const sorted = $derived(
    Object.entries($metrics)
      .filter(([, value]) => value !== 0)
      .sort(([a], [b]) => (a < b ? -1 : a > b ? 1 : 0)),
  )
</script>

<div class={$layoutClass}>
  <details>
    <summary>mDNS-SD-metrics</summary>
    <div class="metrics-grid">
      {#each sorted as [name, value] (name)}
        <div class="metric-item">
          <i>{name}</i>
          <span class="metric-badge">{value}</span>
        </div>
      {/each}
    </div>
  </details>
</div>

<style>
  .metric-item i {
    margin-right: 0.25em;
  }
  .metric-badge {
    display: inline-block;
    padding: 0.1em 0.6em;
    border-radius: 1em;
    font-size: 1.1em;
  }
</style>
