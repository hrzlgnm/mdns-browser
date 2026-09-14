import { writable } from 'svelte/store'

export interface Toast {
  id: number
  title: string
  body: string
}

// Minimal toast stack replacing the thaw `ToasterProvider`. Shown by
// `Toasts.svelte`; call sites gate on the `desktop` store like the Leptos
// `if is_desktop` checks did.
export const toasts = writable<Array<Toast>>([])

let nextId = 1

export function pushToast(title: string, body: string, timeoutMs = 4000) {
  const id = nextId++
  toasts.update((all) => [...all, { id, title, body }])
  setTimeout(() => {
    toasts.update((all) => all.filter((toast) => toast.id !== id))
  }, timeoutMs)
}

export function copyToast(text: string) {
  pushToast('Clipboard', `Copied \`${text}\` to clipboard`)
}
