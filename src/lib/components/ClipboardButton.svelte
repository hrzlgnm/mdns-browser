<script lang="ts">
  import { writeText } from '@tauri-apps/plugin-clipboard-manager'
  import MdiCircle from '~icons/mdi/circle'
  import MdiClipboardText from '~icons/mdi/clipboard-text'
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
    void writeText(value)
    if ($desktop) copyToast(value)
  }
</script>

<button type="button" class={cls} onclick={onClick} title={buttonText}>
  {#if iconOnly}
    <MdiClipboardText class="clipboard-icon" width="1.2em" height="1.2em" aria-hidden="true" />
  {:else if iconClass}
    <span class="clipboard-icon {iconClass}" aria-hidden="true">
      <MdiCircle width="1.2em" height="1.2em" aria-hidden="true" />
    </span>
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
    color: var(--text-secondary);
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
