<script lang="ts">
  import { onMount } from 'svelte'
  import MdiFormatVerticalAlignTop from '~icons/mdi/format-vertical-align-top'

  let { threshold = 300 }: { threshold?: number } = $props()

  let visible = $state(false)

  onMount(() => {
    const onScroll = () => {
      visible = window.scrollY > threshold
    }
    onScroll()
    window.addEventListener('scroll', onScroll, { passive: true })
    return () => window.removeEventListener('scroll', onScroll)
  })

  function scrollTop() {
    window.scrollTo({ top: 0, left: 0, behavior: 'smooth' })
  }
</script>

<div class="back-top-container">
  {#if visible}
    <button type="button" class="back-top-button" onclick={scrollTop} aria-label="Back to top">
      <MdiFormatVerticalAlignTop width="1em" height="1em" aria-hidden="true" />
    </button>
  {/if}
</div>
