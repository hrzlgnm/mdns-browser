<script lang="ts">
  import { onMount } from 'svelte'

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
      <svg
        viewBox="0 0 24 24"
        width="24"
        height="24"
        fill="none"
        stroke="currentColor"
        stroke-width="2"
        stroke-linecap="round"
        stroke-linejoin="round"
        aria-hidden="true"
      >
        <polyline points="18 15 12 9 6 15" />
      </svg>
    </button>
  {/if}
</div>
