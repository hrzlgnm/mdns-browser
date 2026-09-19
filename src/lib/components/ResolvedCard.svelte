<script lang="ts">
  import { onDestroy } from 'svelte'
  import { openUrl } from '@tauri-apps/plugin-opener'
  import MdiCheckAll from '~icons/mdi/check-all'
  import MdiCircle from '~icons/mdi/circle'
  import MdiClose from '~icons/mdi/close'
  import MdiListBox from '~icons/mdi/list-box'
  import MdiOpenInNew from '~icons/mdi/open-in-new'
  import MdiUnfoldMoreVertical from '~icons/mdi/unfold-more-vertical'
  import { verifyInstance } from '$lib/api'
  import {
    addrDisplay,
    addrIpString,
    dropLocalAndTrailingDot,
    dropTrailingDot,
    getInstanceName,
    getOpenUrls,
    toLocalTimestamp,
    txtDisplay,
  } from '$lib/browse-utils'
  import ClipboardButton from '$lib/components/ClipboardButton.svelte'
  import ValuesTable from '$lib/components/ValuesTable.svelte'
  import { cssClass } from '$lib/css'
  import { browsing } from '$lib/store'
  import type { ResolvedService } from '$lib/types'

  // Matches VERIFY_TIMEOUT in src-tauri (5s).
  const VERIFY_TIMEOUT_MS = 5000

  let { service }: { service: ResolvedService } = $props()

  const cardClass = cssClass('resolved-service-card')
  const valueCellClass = cssClass('resolved-service-value-cell')
  const cardTitleClass = cssClass('resolved-service-card-title')

  let showDetails = $state(false)
  let menuOpen = $state(false)
  let verifying = $state(false)
  let verifyTimer: ReturnType<typeof setTimeout> | undefined = undefined

  onDestroy(() => {
    clearTimeout(verifyTimer)
  })

  const title = $derived(getInstanceName(service))
  const urls = $derived(getOpenUrls(service))
  const updatedAt = $derived(toLocalTimestamp(service.updated_at_micros))
  const addrs = $derived(service.addresses.map((addr) => addrDisplay(addr)))
  const addrsForCopy = $derived(service.addresses.map((addr) => addrIpString(addr)))
  const txts = $derived(service.txt.map((record) => txtDisplay(record)))
  const subtype = $derived(service.subtype === null ? [] : [service.subtype])
  const hostnameDisplay = $derived(dropTrailingDot(service.hostname))
  const serviceTypeDisplay = $derived(dropLocalAndTrailingDot(service.service_type))
  const firstAddress = $derived(addrs[0] ?? '')
  const firstAddressForCopy = $derived(addrsForCopy[0] ?? '')
  const firstAddressDisplay = $derived(
    service.addresses.length > 1
      ? `${firstAddress} (+${service.addresses.length - 1})`
      : firstAddress,
  )
  const deadOrAliveClass = $derived(
    service.dead ? 'resolved-service-dead' : 'resolved-service-alive',
  )
  const cannotVerify = $derived(service.dead || !$browsing)

  function onVerify() {
    verifying = true
    void verifyInstance(service.instance_fullname)
    clearTimeout(verifyTimer)
    verifyTimer = setTimeout(() => {
      verifying = false
    }, VERIFY_TIMEOUT_MS)
  }
</script>

<svelte:window
  onkeydown={(e) => {
    if (e.key === 'Escape' && (showDetails || menuOpen)) {
      showDetails = false
      menuOpen = false
    }
  }}
/>

<div class={$cardClass}>
  <div>
    <ClipboardButton
      class={$cardTitleClass}
      text={service.instance_fullname}
      buttonText={title}
      iconClass={deadOrAliveClass}
    />
  </div>
  <table class="card-table">
    <tbody>
      <tr>
        <td><em>Hostname</em></td>
        <td class={$valueCellClass}>
          <ClipboardButton text={service.hostname} buttonText={hostnameDisplay} />
        </td>
      </tr>
      <tr>
        <td><em>Port</em></td>
        <td class={$valueCellClass}>
          <ClipboardButton text={String(service.port)} buttonText={String(service.port)} />
        </td>
      </tr>
      <tr>
        <td><em>Type</em></td>
        <td class={$valueCellClass}>
          <ClipboardButton text={service.service_type} buttonText={serviceTypeDisplay} />
        </td>
      </tr>
      <tr>
        <td><em>IP</em></td>
        <td class={$valueCellClass}>
          <ClipboardButton
            text={firstAddress}
            buttonText={firstAddressDisplay}
            copyText={firstAddressForCopy}
          />
        </td>
      </tr>
      <tr>
        <td><em>Updated at</em></td>
        <td class={$valueCellClass}>
          <ClipboardButton text={updatedAt} buttonText={updatedAt} />
        </td>
      </tr>
      <tr>
        <td>
          <button
            type="button"
            class="themed-button themed-button-small"
            onclick={() => (showDetails = true)}
          >
            <MdiListBox width="1.2em" height="1.2em" aria-hidden="true" />
            Details
          </button>
        </td>
        <td class={$valueCellClass}>
          <button
            type="button"
            class="themed-button themed-button-small"
            onclick={onVerify}
            disabled={cannotVerify}
          >
            {#if verifying}
              <span class="spinner" role="status" aria-label="Verifying"></span>
            {:else}
              <MdiCheckAll width="1.2em" height="1.2em" aria-hidden="true" />
            {/if}
            Verify
          </button>
          <span class="url-menu-container">
            <button
              type="button"
              class="themed-button themed-button-small"
              onclick={() => {
                if (urls.length === 1) void openUrl(urls[0] ?? '')
                else if (urls.length > 1) menuOpen = !menuOpen
              }}
              disabled={urls.length === 0}
            >
              <MdiOpenInNew width="1.2em" height="1.2em" aria-hidden="true" />
              {#if urls.length > 1}
                <MdiUnfoldMoreVertical width="1.2em" height="1.2em" aria-hidden="true" />
              {/if}
              Open
            </button>
            {#if menuOpen}
              <div
                class="url-menu-overlay"
                role="presentation"
                onclick={() => (menuOpen = false)}
              ></div>
              <div class="url-menu" role="menu">
                {#each urls as url (url)}
                  <button
                    type="button"
                    class="url-menu-item"
                    role="menuitem"
                    onclick={() => {
                      menuOpen = false
                      void openUrl(url)
                    }}
                  >
                    {url}
                  </button>
                {/each}
              </div>
            {/if}
          </span>
        </td>
      </tr>
    </tbody>
  </table>
</div>

{#if showDetails}
  <div
    class="dialog-overlay"
    role="presentation"
    onclick={(e) => {
      if (e.target === e.currentTarget) showDetails = false
    }}
  >
    <div
      class="resolved-service-details-dialog-body dialog-panel"
      style:background-color="var(--bg-secondary)"
      style:color="var(--text-primary)"
      role="dialog"
      aria-modal="true"
      aria-label={title}
    >
      <div class="dialog-header">
        <span class={deadOrAliveClass} aria-hidden="true">
          <MdiCircle width="1.2em" height="1.2em" aria-hidden="true" />
        </span>
        <span class="resolved-service-details-dialog-title">{title}</span>
        <button
          type="button"
          class="themed-button themed-button-small"
          onclick={() => (showDetails = false)}
          aria-label="Close details"
        >
          <MdiClose width="1.2em" height="1.2em" aria-hidden="true" />
        </button>
      </div>
      <div class="resolved-service-details-dialog-scrollarea">
        <ValuesTable values={subtype} title="Subtype" />
        <ValuesTable values={addrs} title="IPs" copyValues={addrsForCopy} />
        <ValuesTable values={txts} title="TXT" />
      </div>
    </div>
  </div>
{/if}

<style>
  .url-menu-container {
    position: relative;
    display: inline-block;
  }
  .url-menu {
    position: absolute;
    right: 0;
    top: calc(100% + 4px);
    z-index: 10000;
    min-width: 220px;
    max-width: 340px;
    padding: 4px;
    border: 1px solid var(--border-primary);
    border-radius: 4px;
    background: var(--bg-primary);
    color: var(--text-primary);
    box-shadow: var(--shadow16, 0 4px 16px rgba(0, 0, 0, 0.5));
  }
  .url-menu-item {
    display: block;
    width: 100%;
    padding: 4px 8px;
    border: none;
    background: none;
    color: var(--text-primary);
    font: inherit;
    text-align: left;
    white-space: normal;
    word-break: break-all;
    border-radius: 2px;
    cursor: pointer;
  }
  .url-menu-item:hover,
  .url-menu-item:focus-visible {
    background: var(--bg-secondary);
    color: var(--accent);
    outline: none;
  }
  .url-menu-overlay {
    position: fixed;
    inset: 0;
    z-index: 9999;
  }
  .dialog-overlay {
    position: fixed;
    inset: 0;
    z-index: 9999;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 1rem;
    overflow: auto;
    background: rgba(0, 0, 0, 0.45);
  }
  .dialog-panel {
    border: 1px solid #808080;
    border-radius: 8px;
    padding: 1rem;
  }
  .dialog-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 0.5rem;
  }
  .dialog-header .resolved-service-details-dialog-title {
    flex: 1;
    min-width: 0;
  }
</style>
