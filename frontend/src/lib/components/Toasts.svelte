<script lang="ts">
  import { toasts } from '$lib/toast'
</script>

<div class="toasts" aria-live="polite">
  {#each $toasts as toast (toast.id)}
    <div class="toast">
      <strong>{toast.title}</strong>
      <span>{toast.body}</span>
    </div>
  {/each}
</div>

<style>
  .toasts {
    position: fixed;
    right: 1rem;
    bottom: 1.25rem;
    z-index: 10000;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    max-width: min(90vw, 24rem);
  }
  .toast {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
    padding: 0.75rem 1rem;
    border-radius: 0.5rem;
    background: var(--colorNeutralBackground1, #1b1a19);
    color: var(--colorNeutralForeground1, #fff);
    box-shadow: var(--shadow16, 0 0 8px rgba(0, 0, 0, 0.4));
    overflow-wrap: anywhere;
  }
</style>
