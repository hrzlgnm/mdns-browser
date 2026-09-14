<script lang="ts">
  import { currentTheme, setTheme, systemTheme } from '$lib/store'
  import { themes } from '$lib/themes'
  import type { ThemeName } from '$lib/themes'

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
</script>

<label class="theme-switcher">
  Theme
  <select
    class="themed-select"
    value={$currentTheme}
    onchange={(e) => {
      const t = e.target as HTMLSelectElement
      setTheme(t.value as ThemeName)
    }}
  >
    <option value="system">{$systemTheme === 'dark' ? 'System (Dark)' : 'System (Light)'}</option>
    {#each sortedThemes as theme (theme.name)}
      <option value={theme.name}>{theme.label}</option>
    {/each}
  </select>
</label>

<style>
  .theme-switcher {
    display: inline-flex;
    align-items: center;
    gap: 0.5em;
  }
</style>
