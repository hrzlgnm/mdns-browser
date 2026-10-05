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
    initTutorialSeen,
    isTutorialSeen,
    markTutorialSeen,
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
  const sortOptions: Array<{ value: SortKind; label: string }> = [
    { value: 'InstanceAsc', label: 'Instance (Ascending)' },
    { value: 'InstanceDesc', label: 'Instance (Descending)' },
    { value: 'HostnameAsc', label: 'Hostname (Ascending)' },
    { value: 'HostnameDesc', label: 'Hostname (Descending)' },
    { value: 'PortAsc', label: 'Port (Ascending)' },
    { value: 'PortDesc', label: 'Port (Descending)' },
    { value: 'ServiceTypeAsc', label: 'Service Type (Ascending)' },
    { value: 'ServiceTypeDesc', label: 'Service Type (Descending)' },
    { value: 'IpAddrAsc', label: 'IP (Ascending)' },
    { value: 'IpAddrDesc', label: 'IP (Descending)' },
    { value: 'TimestampAsc', label: 'Last Updated (Ascending)' },
    { value: 'TimestampDesc', label: 'Last Updated (Descending)' },
  ]
  const sortLabel = $derived(sortOptions.find((o) => o.value === sortValue)?.label ?? sortValue)
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
    // The tutorial fires at most once: skip entirely once seen.
    if (isTutorialSeen()) return
    clearFocusTimer()
    focusTimer = setTimeout(() => {
      markTutorialSeen()
      inputEl?.focus()
      // `focus()` is a no-op when already focused, so open explicitly.
      openDropdown()
    }, AUTO_FOCUS_DELAY_MS)
  }

  onMount(() => {
    // Start only after the persisted flag loads, so a stored seen-state
    // cannot lose a race against the timer. Interactions before that
    // resolve mark the flag, which startFocusTimer then honors.
    void initTutorialSeen().then(() => startFocusTimer())
    // The tutorial ends at the first sign of the user: any pointer,
    // keyboard, focus, or scroll interaction stops the auto-focus timer,
    // so it can never steal focus from an in-use control.
    const stopTutorial = () => {
      clearFocusTimer()
      markTutorialSeen()
    }
    window.addEventListener('pointerdown', stopTutorial)
    window.addEventListener('keydown', stopTutorial)
    window.addEventListener('focusin', stopTutorial)
    window.addEventListener('wheel', stopTutorial, { passive: true })
    return () => {
      window.removeEventListener('pointerdown', stopTutorial)
      window.removeEventListener('keydown', stopTutorial)
      window.removeEventListener('focusin', stopTutorial)
      window.removeEventListener('wheel', stopTutorial)
    }
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

  // Keeps the keyboard-navigated option visible; `nearest` scrolls the
  // listbox only, never the page.
  function scrollOptionIntoView(idPrefix: string, index: number) {
    document.getElementById(`${idPrefix}-${index}`)?.scrollIntoView({ block: 'nearest' })
  }

  function onServiceTypeKeydown(e: KeyboardEvent) {
    // Enter confirms an in-progress IME composition (keyCode 229 is the
    // legacy Chromium marker), it must not select or browse.
    if (e.key === 'Enter' && (e.isComposing || e.keyCode === 229)) return
    if (e.key === 'ArrowDown' && dropdownVisible) {
      e.preventDefault()
      activeIndex = (activeIndex + 1) % suggestions.length
      scrollOptionIntoView('service-type-option', activeIndex)
    } else if (e.key === 'ArrowUp' && dropdownVisible) {
      e.preventDefault()
      activeIndex = (activeIndex - 1 + suggestions.length) % suggestions.length
      scrollOptionIntoView('service-type-option', activeIndex)
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

  let sortOpen = $state(false)
  let sortActiveIndex = $state(-1)
  let sortTriggerEl: HTMLDivElement | undefined = $state(undefined)

  function closeSort() {
    sortOpen = false
    sortActiveIndex = -1
  }

  function openSortList() {
    // Any interaction already stops the tutorial timer via the global
    // listeners registered on mount.
    sortOpen = true
    sortActiveIndex = sortOptions.findIndex((o) => o.value === sortValue)
  }

  function chooseSort(value: SortKind) {
    sortValue = value
    closeSort()
    sortTriggerEl?.focus()
  }

  function onSortTriggerKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault()
      if (!sortOpen) {
        openSortList()
      } else {
        const delta = e.key === 'ArrowDown' ? 1 : -1
        sortActiveIndex = (sortActiveIndex + delta + sortOptions.length) % sortOptions.length
        scrollOptionIntoView('sort-option', sortActiveIndex)
      }
    } else if ((e.key === 'Enter' || e.key === ' ') && !sortOpen) {
      e.preventDefault()
      openSortList()
    } else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault()
      const selected = sortOptions[sortActiveIndex]
      if (selected !== undefined) chooseSort(selected.value)
      else closeSort()
    } else if (e.key === 'Escape' && sortOpen) {
      e.preventDefault()
      closeSort()
    }
  }

  function onSortFocusOut(e: FocusEvent) {
    // Keep the listbox open while focus moves to one of its options.
    const next = e.relatedTarget as Node | null
    if (next !== null && e.currentTarget instanceof Node && e.currentTarget.contains(next)) return
    closeSort()
  }
</script>

<div class={$layoutClass}>
  <BackTop threshold={100} />
  <div>
    <ProtocolFlags disabled={$browsing} />
    <div>
      <span
        class={`${inputStateClass} ${$inputClass} themed-combobox`}
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
          <ul id="service-type-listbox" class="themed-listbox" role="listbox">
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
      <span class="sort-label" id="sort-label">Sort by</span>
      <span class="themed-combobox" onfocusout={onSortFocusOut}>
        <div
          role="combobox"
          tabindex="0"
          class="themed-select sort-button"
          bind:this={sortTriggerEl}
          aria-labelledby="sort-label sort-value"
          aria-haspopup="listbox"
          aria-expanded={sortOpen}
          aria-controls="sort-listbox"
          aria-activedescendant={sortActiveIndex >= 0
            ? `sort-option-${sortActiveIndex}`
            : undefined}
          onclick={() => (sortOpen ? closeSort() : openSortList())}
          onkeydown={onSortTriggerKeydown}
        >
          {sortLabel}
        </div>
        <span id="sort-value" class="visually-hidden">{sortLabel}</span>
        {#if sortOpen}
          <ul id="sort-listbox" class="themed-listbox" role="listbox" aria-labelledby="sort-label">
            {#each sortOptions as option, i (option.value)}
              <li
                id={`sort-option-${i}`}
                role="option"
                aria-selected={option.value === sortValue}
                class:active={i === sortActiveIndex}
              >
                <button
                  type="button"
                  tabindex="-1"
                  onmousedown={(e) => e.preventDefault()}
                  onclick={() => chooseSort(option.value)}
                  onmousemove={() => (sortActiveIndex = i)}
                >
                  {option.label}
                </button>
              </li>
            {/each}
          </ul>
        {/if}
      </span>
      <input
        type="text"
        placeholder="Quick filter"
        autocapitalize="none"
        autocomplete="off"
        autocorrect="off"
        spellcheck="false"
        class={`${$inputClass} themed-input`}
        bind:value={query}
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
  .sort-button {
    text-align: left;
  }
  /* The select-style focus ring is suppressed by .themed-select; restore
     a visible indicator since the combobox is keyboard-operated. */
  .sort-button:focus-visible {
    outline: 2px solid var(--accent);
    outline-offset: 1px;
  }
  .themed-listbox li[aria-selected='true'] button {
    font-weight: 600;
  }
</style>
