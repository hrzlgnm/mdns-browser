<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { browseMany, browseTypes, stopBrowse } from '$lib/api'
  import {
    compareServices,
    isValidServiceType,
    matchesQuery,
    serviceTypeMatches,
    type SortKind,
  } from '$lib/browse-utils'
  import BackTop from '$lib/components/BackTop.svelte'
  import ProtocolFlags from '$lib/components/ProtocolFlags.svelte'
  import ResolvedCard from '$lib/components/ResolvedCard.svelte'
  import { cssClass } from '$lib/css'
  import { browsing, hasEnabledInterfaces, resolved, resolvedList, serviceTypes } from '$lib/store'

  // Matches the backend auto-focus delay (5s).
  const AUTO_FOCUS_DELAY_MS = 5000

  const layoutClass = cssClass('browse-layout')
  const inputClass = cssClass('input')
  const gridClass = cssClass('resolved-service-grid')

  let serviceTypeInput = $state('')
  let sortValue = $state<SortKind>('HostnameAsc')
  let query = $state('')
  let inputEl: HTMLInputElement | undefined = $state(undefined)
  let focusTimer: ReturnType<typeof setTimeout> | undefined = undefined

  function clearFocusTimer() {
    clearTimeout(focusTimer)
    focusTimer = undefined
  }

  function startFocusTimer() {
    clearFocusTimer()
    focusTimer = setTimeout(() => {
      inputEl?.focus()
    }, AUTO_FOCUS_DELAY_MS)
  }

  onMount(() => {
    // Stop any previously started browsing so a frontend reload never
    // resumes it, then trigger service-type discovery for the autocomplete.
    void (async () => {
      await stopBrowse()
      await browseTypes()
    })()
    startFocusTimer()
  })

  onDestroy(() => {
    clearFocusTimer()
  })

  const sortedServices = $derived(
    [...$resolvedList].sort((a, b) => compareServices(a, b, sortValue)),
  )
  const filtered = $derived(sortedServices.filter((service) => matchesQuery(service, query)))
  const invalid = $derived(serviceTypeInput !== '' && !isValidServiceType(serviceTypeInput))
  const browseDisabled = $derived($browsing || invalid || !$hasEnabledInterfaces)
  const suggestions = $derived(
    $serviceTypes.filter((st) => serviceTypeMatches(serviceTypeInput, st)),
  )
  const inputStateClass = $derived(invalid ? 'service-type-invalid' : 'service-type-valid')

  // While browsing all, newly discovered service types are browsed too.
  let previousTypes = new Set<string>()
  $effect(() => {
    const current = $serviceTypes
    const added = current.filter((st) => !previousTypes.has(st))
    previousTypes = new Set(current)
    if (added.length > 0 && $browsing && serviceTypeInput === '') {
      console.debug('[mdns-browser] added services while browsing all:', added)
      void browseMany(added)
    }
  })

  function onBrowse() {
    if (browseDisabled) return
    clearFocusTimer()
    resolved.set(new Map())
    browsing.set(true)
    if (serviceTypeInput === '') {
      void browseMany($serviceTypes)
    } else {
      void browseMany([serviceTypeInput])
    }
  }

  function onStop() {
    browsing.set(false)
    void stopBrowse()
    serviceTypeInput = ''
    startFocusTimer()
  }
</script>

<div class={$layoutClass}>
  <BackTop threshold={100} />
  <div>
    <ProtocolFlags disabled={$browsing} />
    <div>
      <span class={`${inputStateClass} ${$inputClass}`}>
        <input
          class="themed-input"
          bind:this={inputEl}
          type="text"
          placeholder="Service type..."
          autocapitalize="none"
          autocomplete="off"
          autocorrect="off"
          spellcheck="false"
          list="service-type-suggestions"
          bind:value={serviceTypeInput}
          disabled={$browsing}
          onkeydown={(e) => {
            if (e.key === 'Enter' && !invalid) onBrowse()
          }}
        />
        <datalist id="service-type-suggestions">
          {#each suggestions as suggestion (suggestion)}
            <option value={suggestion}></option>
          {/each}
        </datalist>
      </span>
      <button type="button" class="themed-button" onclick={onBrowse} disabled={browseDisabled}>
        Browse
      </button>
      <button type="button" class="themed-button" onclick={onStop} disabled={!$browsing}>
        Stop
      </button>
      <span class="themed-badge">{filtered.length}/{sortedServices.length}</span>
    </div>
    <div>
      <span class="sort-label">Sort by</span>
      <select class="themed-select" bind:value={sortValue}>
        <option value="InstanceAsc">Instance (Ascending)</option>
        <option value="InstanceDesc">Instance (Descending)</option>
        <option value="HostnameAsc">Hostname (Ascending)</option>
        <option value="HostnameDesc">Hostname (Descending)</option>
        <option value="PortAsc">Port (Ascending)</option>
        <option value="PortDesc">Port (Descending)</option>
        <option value="ServiceTypeAsc">Service Type (Ascending)</option>
        <option value="ServiceTypeDesc">Service Type (Descending)</option>
        <option value="IpAddrAsc">IP (Ascending)</option>
        <option value="IpAddrDesc">IP (Descending)</option>
        <option value="TimestampAsc">Last Updated (Ascending)</option>
        <option value="TimestampDesc">Last Updated (Descending)</option>
      </select>
      <input
        type="text"
        placeholder="Quick filter"
        autocapitalize="none"
        autocomplete="off"
        autocorrect="off"
        spellcheck="false"
        class={`${$inputClass} themed-input`}
        bind:value={query}
        onfocus={() => clearFocusTimer()}
        onkeydown={(e) => {
          if (e.key === 'Enter') onBrowse()
        }}
      />
    </div>
  </div>
  <div class={$gridClass}>
    {#each filtered as service (service.instance_fullname)}
      <ResolvedCard {service} />
    {/each}
  </div>
</div>
