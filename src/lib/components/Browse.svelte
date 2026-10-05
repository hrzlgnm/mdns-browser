<script lang="ts">
  import { onDestroy, onMount } from 'svelte'
  import { get } from 'svelte/store'
  import { browseMany, stopBrowse } from '#lib/api.js'
  import {
    compareServices,
    isValidServiceType,
    matchesQuery,
    serviceTypeMatches,
    type SortKind,
  } from '#lib/browse-utils.js'
  import BackTop from '#lib/components/BackTop.svelte'
  import ProtocolFlags from '#lib/components/ProtocolFlags.svelte'
  import ResolvedCard from '#lib/components/ResolvedCard.svelte'
  import { cssClass } from '#lib/css.js'
  import {
    browsing,
    hasEnabledInterfaces,
    initLocalNetworkAccess,
    resolved,
    resolvedList,
    serviceTypes,
  } from '#lib/store.js'

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
  // Native datalists cannot be opened programmatically (`showPicker()`
  // needs a user gesture and is unsupported in Firefox/Safari), so the
  // suggestions render as a custom listbox that focus can expand.
  let dropdownOpen = $state(false)
  let activeIndex = $state(-1)

  function clearFocusTimer() {
    clearTimeout(focusTimer)
    focusTimer = undefined
  }

  function openDropdown() {
    dropdownOpen = true
  }

  function closeDropdown() {
    dropdownOpen = false
    activeIndex = -1
  }

  function startFocusTimer() {
    clearFocusTimer()
    focusTimer = setTimeout(() => {
      inputEl?.focus()
      // `focus()` is a no-op when already focused, so open explicitly.
      openDropdown()
    }, AUTO_FOCUS_DELAY_MS)
  }

  onMount(() => {
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
  const dropdownVisible = $derived(dropdownOpen && suggestions.length > 0 && !$browsing)
  // Keep keyboard navigation inside the filtered list as it changes.
  $effect(() => {
    void suggestions.length
    if (activeIndex >= suggestions.length) activeIndex = suggestions.length - 1
  })
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

  function selectSuggestion(suggestion: string) {
    serviceTypeInput = suggestion
    closeDropdown()
    inputEl?.focus()
  }

  function onServiceTypeKeydown(e: KeyboardEvent) {
    // Enter confirms an in-progress IME composition (keyCode 229 is the
    // legacy Chromium marker), it must not select or browse.
    if (e.key === 'Enter' && (e.isComposing || e.keyCode === 229)) return
    if (e.key === 'ArrowDown' && dropdownVisible) {
      e.preventDefault()
      activeIndex = (activeIndex + 1) % suggestions.length
    } else if (e.key === 'ArrowUp' && dropdownVisible) {
      e.preventDefault()
      activeIndex = (activeIndex - 1 + suggestions.length) % suggestions.length
    } else if (e.key === 'Enter' && !invalid) {
      if (dropdownVisible && activeIndex >= 0 && activeIndex < suggestions.length) {
        e.preventDefault()
        const selected = suggestions[activeIndex]
        if (selected !== undefined) selectSuggestion(selected)
      } else {
        onBrowse()
      }
    } else if (e.key === 'Escape' && dropdownOpen) {
      e.preventDefault()
      closeDropdown()
    }
  }

  function onBrowse() {
    if (browseDisabled) return
    clearFocusTimer()
    closeDropdown()
    resolved.set(new Map())
    browsing.set(true)
    void (async () => {
      // Re-check access: a revocation mid-session silently starves
      // discovery, so surface the blocking panel instead of browsing.
      if ((await initLocalNetworkAccess()) !== 'granted') {
        browsing.set(false)
        return
      }
      // The user may have pressed Stop while the access check was pending.
      if (!get(browsing)) return
      if (serviceTypeInput === '') {
        void browseMany($serviceTypes)
      } else {
        void browseMany([serviceTypeInput])
      }
    })()
  }

  function onStop() {
    browsing.set(false)
    void stopBrowse()
    serviceTypeInput = ''
    closeDropdown()
    startFocusTimer()
  }

  function onServiceTypeFocusOut(e: FocusEvent) {
    // Keep the listbox open while focus moves to one of its options.
    const next = e.relatedTarget as Node | null
    if (next !== null && e.currentTarget instanceof Node && e.currentTarget.contains(next)) return
    closeDropdown()
  }
</script>

<div class={$layoutClass}>
  <BackTop threshold={100} />
  <div>
    <ProtocolFlags disabled={$browsing} />
    <div>
      <span
        class={`${inputStateClass} ${$inputClass} service-type-combobox`}
        onfocusout={onServiceTypeFocusOut}
      >
        <input
          class="themed-input"
          bind:this={inputEl}
          type="text"
          role="combobox"
          aria-expanded={dropdownVisible}
          aria-controls="service-type-listbox"
          aria-activedescendant={activeIndex >= 0
            ? `service-type-option-${activeIndex}`
            : undefined}
          aria-autocomplete="list"
          placeholder="Service type..."
          autocapitalize="none"
          autocomplete="off"
          autocorrect="off"
          spellcheck="false"
          bind:value={serviceTypeInput}
          disabled={$browsing}
          onfocus={openDropdown}
          oninput={() => {
            activeIndex = -1
            openDropdown()
          }}
          onkeydown={onServiceTypeKeydown}
        />
        {#if dropdownVisible}
          <ul id="service-type-listbox" class="service-type-listbox" role="listbox">
            {#each suggestions as suggestion, i (suggestion)}
              <li
                id={`service-type-option-${i}`}
                role="option"
                aria-selected={i === activeIndex}
                class:active={i === activeIndex}
              >
                <button
                  type="button"
                  tabindex="-1"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => selectSuggestion(suggestion)}
                  onmousemove={() => (activeIndex = i)}
                >
                  {suggestion}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
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

<style>
  .service-type-combobox {
    position: relative;
    display: inline-block;
  }
  .service-type-listbox {
    position: absolute;
    z-index: 100;
    top: 100%;
    left: 0;
    right: 0;
    margin: 2px 0 0;
    padding: 0;
    list-style: none;
    max-height: 16rem;
    overflow-y: auto;
    background: var(--bg-primary);
    border: 1px solid var(--border-primary);
    border-radius: 4px;
    box-shadow: var(--shadow-elevated);
  }
  .service-type-listbox button {
    display: block;
    width: 100%;
    padding: 4px 8px;
    font: inherit;
    font-size: 14px;
    text-align: left;
    background: transparent;
    color: var(--text-primary);
    border: 0;
    cursor: pointer;
  }
  .service-type-listbox li.active button,
  .service-type-listbox button:hover {
    background: var(--bg-tertiary);
  }
</style>
