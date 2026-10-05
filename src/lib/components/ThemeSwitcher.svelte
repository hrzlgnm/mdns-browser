<script lang="ts">
  import { currentTheme, setTheme, systemTheme } from '#lib/store.js'
  import { themes } from '#lib/themes.js'
  import type { ThemeName } from '#lib/themes.js'

  // Initialized and kept in sync by `initTheme` (called from Main);
  // this component only changes the selection.
  const sortedThemes = (() => {
    const byName = new Map(themes.map((t) => [t.name, t]))
    const dark = byName.get('dark')
    const light = byName.get('light')
    const rest = themes
      .filter((t) => t.name !== 'dark' && t.name !== 'light')
      .sort((a, b) => a.label.localeCompare(b.label))
    return [dark, light, ...rest].filter((t): t is (typeof themes)[number] => Boolean(t))
  })()

  interface ThemeOption {
    value: ThemeName
    label: string
  }

  const options = $derived<ThemeOption[]>([
    { value: 'system', label: $systemTheme === 'dark' ? 'System (Dark)' : 'System (Light)' },
    ...sortedThemes.map((t) => ({ value: t.name as ThemeName, label: t.label })),
  ])
  const selectedLabel = $derived(
    options.find((o) => o.value === $currentTheme)?.label ?? $currentTheme,
  )

  let open = $state(false)
  let activeIndex = $state(-1)
  let buttonEl: HTMLButtonElement | undefined = $state(undefined)

  function close() {
    open = false
    activeIndex = -1
  }

  function openList() {
    open = true
    activeIndex = options.findIndex((o) => o.value === $currentTheme)
  }

  function choose(value: ThemeName) {
    setTheme(value)
    close()
    buttonEl?.focus()
  }

  function onButtonKeydown(e: KeyboardEvent) {
    if (e.key === 'ArrowDown' || e.key === 'ArrowUp') {
      e.preventDefault()
      if (!open) {
        openList()
      } else {
        const delta = e.key === 'ArrowDown' ? 1 : -1
        activeIndex = (activeIndex + delta + options.length) % options.length
      }
    } else if ((e.key === 'Enter' || e.key === ' ') && !open) {
      e.preventDefault()
      openList()
    } else if (e.key === 'Enter' || e.key === ' ') {
      e.preventDefault()
      const selected = options[activeIndex]
      if (selected !== undefined) choose(selected.value)
      else close()
    } else if (e.key === 'Escape' && open) {
      e.preventDefault()
      close()
    }
  }

  function onFocusOut(e: FocusEvent) {
    // Keep the listbox open while focus moves to one of its options.
    const next = e.relatedTarget as Node | null
    if (next !== null && e.currentTarget instanceof Node && e.currentTarget.contains(next)) return
    close()
  }
</script>

<span class="theme-switcher themed-combobox" onfocusout={onFocusOut}>
  <span id="theme-label">Theme</span>
  <button
    type="button"
    class="themed-select theme-button"
    bind:this={buttonEl}
    aria-labelledby="theme-label"
    aria-haspopup="listbox"
    aria-expanded={open}
    aria-controls="theme-listbox"
    onclick={() => (open ? close() : openList())}
    onkeydown={onButtonKeydown}
  >
    {selectedLabel}
  </button>
  {#if open}
    <ul id="theme-listbox" class="themed-listbox" role="listbox" aria-labelledby="theme-label">
      {#each options as option, i (option.value)}
        <li
          id={`theme-option-${i}`}
          role="option"
          aria-selected={option.value === $currentTheme}
          class:active={i === activeIndex}
        >
          <button
            type="button"
            tabindex="-1"
            onmousedown={(e) => e.preventDefault()}
            onclick={() => choose(option.value)}
            onmousemove={() => (activeIndex = i)}
          >
            {option.label}
          </button>
        </li>
      {/each}
    </ul>
  {/if}
</span>

<style>
  .theme-switcher {
    display: inline-flex;
    align-items: center;
    gap: 0.5em;
  }
  .theme-switcher > .theme-button {
    text-align: left;
  }
  .themed-listbox li[aria-selected='true'] button {
    font-weight: 600;
  }
</style>
