<script lang="ts">
  import { copyToClipboard } from '$lib/api'
  import { desktop } from '$lib/store'
  import { copyToast } from '$lib/toast'

  let {
    text,
    buttonText,
    copyText,
    class: cls = '',
    iconClass = '',
    iconOnly = false,
  }: {
    text: string
    buttonText: string
    copyText?: string
    class?: string
    iconClass?: string
    iconOnly?: boolean
  } = $props()

  function onClick() {
    const value = copyText ?? text
    void copyToClipboard(value)
    if ($desktop) copyToast(value)
  }
</script>

<button type="button" class={cls} onclick={onClick} title={buttonText}>
  {#if iconOnly}
    <svg
      class="clipboard-icon"
      viewBox="0 0 24 24"
      width="1em"
      height="1em"
      fill="none"
      stroke="currentColor"
      stroke-width="2"
      stroke-linecap="round"
      stroke-linejoin="round"
      aria-hidden="true"
    >
      <rect x="9" y="9" width="13" height="13" rx="2" ry="2" />
      <path d="M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" />
    </svg>
  {:else if iconClass}
    <span class="clipboard-icon {iconClass}" aria-hidden="true">●</span>
  {/if}
  {#if !iconOnly}
    <span class="clipboard-text">{buttonText}</span>
  {/if}
</button>

<style>
  button {
    display: inline-flex;
    align-items: center;
    gap: 0.25rem;
    max-width: 100%;
    overflow: hidden;
    background: transparent;
    border: 1px solid transparent;
    border-radius: 4px;
    padding: 1px 4px;
    color: var(--text-muted);
    cursor: pointer;
  }
  button:hover {
    background: var(--bg-tertiary);
    border-color: var(--border-primary);
    color: var(--text-primary);
  }
  .clipboard-icon {
    flex-shrink: 0;
  }
  .clipboard-text {
    min-width: 0;
    overflow: hidden;
    white-space: nowrap;
    text-overflow: ellipsis;
  }
</style>
