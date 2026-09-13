import { derived, writable } from 'svelte/store'
import type { UnlistenFn } from '@tauri-apps/api/event'
import {
  getProtocolFlags,
  getTheme,
  isDesktop,
  onInterfacesChanged,
  onMetricsChanged,
  onServiceRemoved,
  onServiceResolved,
  onServiceTypeFound,
  onThemeChanged,
  subscribeInterfaces,
  subscribeMetrics,
} from './api'
import type { NetworkInterface, ProtocolFlags, ResolvedService, ServiceTypes } from './types'

// Svelte stores replacing the Leptos signals/contexts in src/app/.
// Event payloads flow backend -> api.ts listen helpers -> these stores.

export const serviceTypes = writable<ServiceTypes>([])
export const resolved = writable<Map<string, ResolvedService>>(new Map())
export const interfaces = writable<Array<NetworkInterface>>([])
export const metrics = writable<Record<string, number>>({})
export const protocolFlags = writable<ProtocolFlags>({ ipv4: true, ipv6: true })
export const theme = writable<string>('dark')
export const desktop = writable<boolean>(true)
export const browsing = writable<boolean>(false)

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
  try {
    theme.set(await getTheme())
  } catch (e) {
    console.warn('[mdns-browser] failed to query theme:', e)
  }
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
    await onThemeChanged((payload) => theme.set(payload.theme)),
  ]
  // Subscribe after listening so no initial emission is missed.
  await subscribeInterfaces()
  await subscribeMetrics()
  return unlisteners
}
