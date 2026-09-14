import { derived, get, writable } from 'svelte/store'
import type { UnlistenFn } from '@tauri-apps/api/event'
import { isTauri } from '@tauri-apps/api/core'
import { Store } from '@tauri-apps/plugin-store'
import {
  getProtocolFlags,
  isDesktop,
  onInterfacesChanged,
  onMetricsChanged,
  onServiceRemoved,
  onServiceResolved,
  onServiceTypeFound,
  subscribeInterfaces,
  subscribeMetrics,
} from './api'
import type { NetworkInterface, ProtocolFlags, ResolvedService, ServiceTypes } from './types'
import { cssVarMap, defaultTheme, getThemeByName, isDarkTheme, themes } from './themes'
import type { ThemeColors, ThemeName } from './themes'

// Svelte stores replacing the Leptos signals/contexts in src/app/.
// Event payloads flow backend -> api.ts listen helpers -> these stores.

export const serviceTypes = writable<ServiceTypes>([])
export const resolved = writable<Map<string, ResolvedService>>(new Map())
export const interfaces = writable<Array<NetworkInterface>>([])
export const metrics = writable<Record<string, number>>({})
export const protocolFlags = writable<ProtocolFlags>({ ipv4: true, ipv6: true })
export const desktop = writable<boolean>(true)
export const browsing = writable<boolean>(false)

// ── Theme ──────────────────────────────────────────────────────────────────

export const currentTheme = writable<ThemeName>(defaultTheme)

function detectSystemTheme(): 'dark' | 'light' {
  if (typeof window !== 'undefined' && typeof window.matchMedia === 'function') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light'
  }
  return 'dark'
}

export const systemTheme = writable<'dark' | 'light'>(detectSystemTheme())

export const resolvedTheme = derived([currentTheme, systemTheme], ([$curr, $sys]) => {
  if ($curr === 'system') return getThemeByName($sys)
  return getThemeByName($curr)
})

function applyThemeToCss(colors: ThemeColors, resolvedName: string) {
  if (typeof document === 'undefined') return
  const root = document.documentElement
  for (const [key, varName] of Object.entries(cssVarMap) as [keyof ThemeColors, string][]) {
    root.style.setProperty(varName, colors[key])
  }
  root.style.colorScheme = isDarkTheme(resolvedName) ? 'dark' : 'light'
}

async function syncWindowTheme(resolvedName: string) {
  if (!isTauri()) return
  const target: 'dark' | 'light' = isDarkTheme(resolvedName) ? 'dark' : 'light'
  try {
    const { getCurrentWindow } = await import('@tauri-apps/api/window')
    await getCurrentWindow().setTheme(target)
  } catch (e) {
    console.warn('[mdns-browser] failed to sync window theme:', e)
  }
  try {
    const { setTheme } = await import('@tauri-apps/api/app')
    await setTheme(target)
  } catch {
    // app theme is best-effort; window theme is primary
  }
}

const THEME_STORE_FILE = 'theme-config.json'
const THEME_CONFIG_KEY = 'theme'

let themeStorePromise: Promise<Store> | null = null
let themeSaveChain: Promise<void> = Promise.resolve()

function loadThemeStore(): Promise<Store> {
  if (!themeStorePromise) {
    themeStorePromise = Store.load(THEME_STORE_FILE)
  }
  return themeStorePromise
}

function persistTheme(name: ThemeName) {
  if (!isTauri()) return
  themeSaveChain = themeSaveChain.then(async () => {
    try {
      const store = await loadThemeStore()
      await store.set(THEME_CONFIG_KEY, name)
      await store.save()
    } catch (e) {
      console.error('[mdns-browser] failed to save theme config:', e)
    }
  })
}

let systemListenerCleanup: (() => void) | null = null

function setupSystemThemeListener() {
  if (typeof window === 'undefined' || typeof window.matchMedia !== 'function') return
  if (systemListenerCleanup) return
  const media = window.matchMedia('(prefers-color-scheme: dark)')
  const handler = (e: MediaQueryListEvent | MediaQueryList) => {
    const isDark = 'matches' in e ? e.matches : (e as MediaQueryList).matches
    systemTheme.set(isDark ? 'dark' : 'light')
  }
  if (typeof media.addEventListener === 'function') {
    const listener = (e: MediaQueryListEvent) => handler(e)
    media.addEventListener('change', listener)
    systemListenerCleanup = () => media.removeEventListener('change', listener)
  } else if (
    typeof (media as unknown as { addListener: (cb: (m: MediaQueryList) => void) => void })
      .addListener === 'function'
  ) {
    const m = media as unknown as {
      addListener: (cb: (m: MediaQueryList) => void) => void
      removeListener: (cb: (m: MediaQueryList) => void) => void
    }
    const listener = (m: MediaQueryList) => handler(m)
    m.addListener(listener)
    systemListenerCleanup = () => m.removeListener(listener)
  }
}

export const resolvedList = derived(resolved, ($resolved) => [...$resolved.values()])

export const hasEnabledInterfaces = derived(interfaces, ($interfaces) =>
  $interfaces.some((iface) => iface.enabled),
)

function addServiceType(serviceType: string) {
  serviceTypes.update((types) => (types.includes(serviceType) ? types : [...types, serviceType]))
}

function putResolved(service: ResolvedService) {
  resolved.update((services) => {
    services.set(service.instance_fullname, service)
    return services
  })
}

function markRemoved(instanceName: string, atMicros: string) {
  resolved.update((services) => {
    const service = services.get(instanceName)
    if (service && !service.dead) {
      services.set(instanceName, { ...service, dead: true, updated_at_micros: atMicros })
    }
    return services
  })
}

export async function initDesktop() {
  try {
    desktop.set(await isDesktop())
  } catch (e) {
    console.warn('[mdns-browser] failed to query is_desktop:', e)
  }
}

export async function initTheme() {
  setupSystemThemeListener()
  const initialResolved = get(resolvedTheme)
  applyThemeToCss(initialResolved.colors, initialResolved.name)
  void syncWindowTheme(initialResolved.name)

  resolvedTheme.subscribe((preset) => {
    applyThemeToCss(preset.colors, preset.name)
    void syncWindowTheme(preset.name)
  })

  let firstEmit = true
  currentTheme.subscribe((name) => {
    if (firstEmit) {
      firstEmit = false
      return
    }
    persistTheme(name)
  })
  if (!isTauri()) return
  try {
    const store = await loadThemeStore()
    const saved = await store.get<ThemeName>(THEME_CONFIG_KEY)
    const isValid =
      saved === 'system' || (typeof saved === 'string' && themes.some((t) => t.name === saved))
    if (isValid && saved) {
      currentTheme.set(saved)
    }
  } catch (e) {
    console.error('[mdns-browser] failed to load theme config:', e)
  }
}

export function setTheme(name: ThemeName) {
  currentTheme.set(name)
}

export async function initProtocolFlags() {
  try {
    protocolFlags.set(await getProtocolFlags())
  } catch (e) {
    console.warn('[mdns-browser] failed to query protocol flags:', e)
  }
}

export async function setupEventListeners(): Promise<Array<UnlistenFn>> {
  console.debug('[mdns-browser] setting up event listeners')
  const unlisteners: Array<UnlistenFn> = [
    await onServiceTypeFound((payload) => addServiceType(payload.service_type)),
    await onServiceResolved((payload) => putResolved(payload.service)),
    await onServiceRemoved((payload) => markRemoved(payload.instance_name, payload.at_micros)),
    await onInterfacesChanged((payload) => interfaces.set(payload.interfaces)),
    await onMetricsChanged((payload) => metrics.set(payload.metrics)),
  ]
  // Subscribe after listening so no initial emission is missed.
  await subscribeInterfaces()
  await subscribeMetrics()
  return unlisteners
}
