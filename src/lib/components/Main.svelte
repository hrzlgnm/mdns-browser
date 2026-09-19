<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { browseTypes, closeSplashscreen, stopBrowse } from '$lib/api'
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
    initProtocolFlags,
    initTheme,
    setupEventListeners,
  } from '$lib/store'

  const layoutClass = cssClass('outer-layout')

  let unlisteners: Array<UnlistenFn> = []

  function shouldBlock(event: Event): boolean {
    const target = event.target as HTMLElement | null
    if (target === null) return true
    const tagName = target.tagName.toLowerCase()
    return !(tagName === 'input' || tagName === 'textarea' || target.isContentEditable)
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
</style>
