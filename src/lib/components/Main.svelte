<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { browseTypes, closeSplashscreen, requestLocalNetworkAccess, stopBrowse } from '$lib/api'
  import About from '$lib/components/About.svelte'
  import Browse from '$lib/components/Browse.svelte'
  import Metrics from '$lib/components/Metrics.svelte'
  import NetworkInterfaces from '$lib/components/NetworkInterfaces.svelte'
  import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte'
  import Toasts from '$lib/components/Toasts.svelte'
  import { cssClass } from '$lib/css'
  import { initLogger } from '$lib/logger'
  import {
    browsing,
    initDesktop,
    initLocalNetworkAccess,
    initProtocolFlags,
    initTheme,
    localNetworkAccess,
    setupEventListeners,
  } from '$lib/store'

  const layoutClass = cssClass('outer-layout')

  let unlisteners: Array<UnlistenFn> = []
  let requesting = $state(false)

  function shouldBlock(event: Event): boolean {
    const target = event.target as HTMLElement | null
    if (target === null) return true
    const tagName = target.tagName.toLowerCase()
    return !(tagName === 'input' || tagName === 'textarea' || target.isContentEditable)
  }

  async function startDiscovery() {
    try {
      // Stop any previously started browsing so a reload never resumes it,
      // then trigger service-type discovery. This must happen after the
      // service-type-found listener exists, otherwise the initial
      // _services._dns-sd._udp.local. answers are dropped and the types
      // never show up.
      await stopBrowse()
      await browseTypes()
    } catch (e) {
      console.warn('[mdns-browser] failed to start service type discovery:', e)
    }
  }

  async function onGrantAccess() {
    requesting = true
    try {
      const state = await requestLocalNetworkAccess()
      localNetworkAccess.set(state)
      if (state === 'granted') {
        await startDiscovery()
      }
    } catch (e) {
      console.warn('[mdns-browser] failed to request local network access:', e)
    } finally {
      requesting = false
    }
  }

  async function onRetryAccess() {
    const state = await initLocalNetworkAccess()
    if (state === 'granted') {
      await startDiscovery()
    }
  }

  onMount(() => {
    // Block drop navigation for http links dropped outside editable fields.
    const onDragOver = (e: DragEvent) => {
      if (shouldBlock(e)) e.preventDefault()
    }
    const onDrop = (e: DragEvent) => {
      if (shouldBlock(e)) e.preventDefault()
    }
    window.addEventListener('dragover', onDragOver)
    window.addEventListener('drop', onDrop)

    void (async () => {
      const unlistenLogger = await initLogger()
      await initDesktop()
      await initTheme()
      await initProtocolFlags()
      unlisteners = [unlistenLogger, ...(await setupEventListeners())]
      // Discovery needs local-network access on Android 17+; anything
      // but granted renders the blocking panel below instead.
      if ((await initLocalNetworkAccess()) === 'granted') {
        await startDiscovery()
      }
      await closeSplashscreen()
    })()

    return () => {
      window.removeEventListener('dragover', onDragOver)
      window.removeEventListener('drop', onDrop)
    }
  })

  onDestroy(() => {
    for (const unlisten of unlisteners) unlisten()
    unlisteners = []
  })
</script>

{#if $localNetworkAccess === 'granted'}
  <div class={$layoutClass}>
    <div class="top-row">
      <About />
      <div class="theme-slot">
        <ThemeSwitcher />
      </div>
    </div>
    <Metrics />
    <NetworkInterfaces disabled={$browsing} />
    <Browse />
  </div>
{:else}
  <div class={$layoutClass}>
    <div class="top-row">
      <About />
      <div class="theme-slot">
        <ThemeSwitcher />
      </div>
    </div>
    <div class="access-panel">
      <h2>Local network access needed</h2>
      <p>
        mDNS browsing discovers devices on your local network, which requires nearby-devices access.
        Grant it to start discovering services.
      </p>
      {#if $localNetworkAccess === 'denied'}
        <p>
          Access was denied. Grant it in Settings &gt; Apps &gt; mDNS Browser &gt; Permissions &gt;
          Nearby devices, then retry.
        </p>
      {/if}
      <div class="access-actions">
        <button type="button" class="themed-button" onclick={onGrantAccess} disabled={requesting}>
          {requesting ? 'Requesting…' : 'Grant access'}
        </button>
        <button type="button" class="themed-button" onclick={onRetryAccess}> Retry </button>
      </div>
    </div>
  </div>
{/if}
<Toasts />

<style>
  .top-row {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.5rem;
    align-items: start;
  }
  .theme-slot {
    display: flex;
    justify-content: flex-end;
  }
  .access-panel {
    display: flex;
    flex-direction: column;
    gap: 0.75rem;
    max-width: 36rem;
    padding: 1rem;
  }
  .access-actions {
    display: flex;
    gap: 0.5rem;
  }
</style>
