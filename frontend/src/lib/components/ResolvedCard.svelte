<script lang="ts">
  import { onDestroy } from 'svelte'
  import { openUrl, verifyInstance } from '$lib/api'
  import {
    addrDisplay,
    addrIpString,
    dropLocalAndTrailingDot,
    dropTrailingDot,
    getInstanceName,
    getOpenUrl,
    toLocalTimestamp,
    txtDisplay,
  } from '$lib/browse-utils'
  import ClipboardButton from '$lib/components/ClipboardButton.svelte'
  import ValuesTable from '$lib/components/ValuesTable.svelte'
  import { cssClass } from '$lib/css'
  import { browsing } from '$lib/store'
  import type { ResolvedService } from '$lib/types'

  // VERIFY_TIMEOUT in crates/shared_constants is 5s.
  const VERIFY_TIMEOUT_MS = 5000

  let { service }: { service: ResolvedService } = $props()

  const cardClass = cssClass('resolved-service-card')
  const valueCellClass = cssClass('resolved-service-value-cell')
  const cardTitleClass = cssClass('resolved-service-card-title')

  let showDetails = $state(false)
  let verifying = $state(false)
  let verifyTimer: ReturnType<typeof setTimeout> | undefined = undefined

  onDestroy(() => {
    clearTimeout(verifyTimer)
  })

  const title = $derived(getInstanceName(service))
  const url = $derived(getOpenUrl(service))
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

<div class={$cardClass}>
  <div>
    <ClipboardButton
      class={$cardTitleClass}
      text={service.instance_fullname}
      buttonText={title}
      iconClass={deadOrAliveClass}
    />
  </div>
  <table>
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
          <button type="button" onclick={() => (showDetails = true)}>Details</button>
        </td>
        <td class={$valueCellClass}>
          <button type="button" onclick={onVerify} disabled={cannotVerify}>
            {verifying ? 'Verifying…' : 'Verify'}
          </button>
          <button
            type="button"
            onclick={() => {
              if (url !== null) void openUrl(url)
            }}
            disabled={url === null}
          >
            Open
          </button>
        </td>
      </tr>
    </tbody>
  </table>
</div>

{#if showDetails}
  <div
    class="resolved-service-details-dialog-body"
    role="dialog"
    aria-modal="true"
    aria-label={title}
  >
    <div>
      <span class={deadOrAliveClass} aria-hidden="true">●</span>
      <span class="resolved-service-details-dialog-title">{title}</span>
      <button type="button" onclick={() => (showDetails = false)} aria-label="Close details">
        ✕
      </button>
    </div>
    <div class="resolved-service-details-dialog-scrollarea">
      <ValuesTable values={subtype} title="Subtype" />
      <ValuesTable values={addrs} title="IPs" copyValues={addrsForCopy} />
      <ValuesTable values={txts} title="TXT" />
    </div>
  </div>
{/if}
