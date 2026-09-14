<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import type { UnlistenFn } from '@tauri-apps/api/event'
  import { closeSplashscreen } from '$lib/api'
  import About from '$lib/components/About.svelte'
  import Browse from '$lib/components/Browse.svelte'
  import Metrics from '$lib/components/Metrics.svelte'
  import NetworkInterfaces from '$lib/components/NetworkInterfaces.svelte'
  import ThemeSwitcher from '$lib/components/ThemeSwitcher.svelte'
  import Toasts from '$lib/components/Toasts.svelte'
  import { cssClass } from '$lib/css'
  import {
    browsing,
    initDesktop,
    initProtocolFlags,
    initTheme,
    setupEventListeners,
    theme,
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
    // Keep the thaw dark/light page background behavior without thaw:
    // sync the body background with the current theme name.
    $effect(() => {
      document.body.style.backgroundColor = $theme === 'dark' ? '#1b1a19' : '#ffffff'
    })

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
      await initDesktop()
      await initTheme()
      await initProtocolFlags()
      unlisteners = await setupEventListeners()
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
