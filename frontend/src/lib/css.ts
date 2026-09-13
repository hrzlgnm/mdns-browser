import { derived, type Readable } from 'svelte/store'
import { desktop } from './store'

// Mirrors `get_class` in the former Leptos `src/app/css.rs`: the base class
// plus the `desktop-`/`mobile-` prefixed variant, e.g. `"input desktop-input"`.
export function cssClass(base: string): Readable<string> {
  return derived(desktop, ($desktop) => `${base} ${$desktop ? 'desktop' : 'mobile'}-${base}`)
}
