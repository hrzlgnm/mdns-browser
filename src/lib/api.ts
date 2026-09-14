import { invoke } from '@tauri-apps/api/core'
import { listen, type UnlistenFn } from '@tauri-apps/api/event'
import type {
  InterfacesChangedEvent,
  MetricsChangedEvent,
  ProtocolFlags,
  ServiceRemovedEvent,
  ServiceResolvedEvent,
  ServiceTypeFoundEvent,
  ServiceTypes,
  ThemeChangedEvent,
} from './types'

// Typed wrappers for the Tauri commands in src-tauri/src/lib.rs.
// Tauri camelCases invoke args, so `service_types` becomes `serviceTypes`.

export function browseMany(serviceTypes: ServiceTypes): Promise<void> {
  return invoke<void>('browse_many', { serviceTypes })
}

export function browseTypes(): Promise<void> {
  return invoke<void>('browse_types')
}

export function stopBrowse(): Promise<void> {
  return invoke<void>('stop_browse')
}

export function verifyInstance(instanceFullname: string): Promise<void> {
  return invoke<void>('verify', { instanceFullname })
}

export function subscribeInterfaces(): Promise<void> {
  return invoke<void>('subscribe_interfaces')
}

export function subscribeMetrics(): Promise<void> {
  return invoke<void>('subscribe_metrics')
}

export function openUrl(url: string): Promise<void> {
  return invoke<void>('open_url', { url })
}

export function getVersion(): Promise<string> {
  return invoke<string>('version')
}

export function getProtocolFlags(): Promise<ProtocolFlags> {
  return invoke<ProtocolFlags>('get_protocol_flags')
}

export function setProtocolFlags(flags: ProtocolFlags): Promise<void> {
  return invoke<void>('set_protocol_flags', { flags })
}

export function setInterfaces(enabled: Array<string>): Promise<void> {
  return invoke<void>('set_interfaces', { enabled })
}

export function isDesktop(): Promise<boolean> {
  return invoke<boolean>('is_desktop')
}

export function copyToClipboard(contents: string): Promise<void> {
  return invoke<void>('copy_to_clipboard', { contents })
}

export function getTheme(): Promise<string> {
  return invoke<string>('theme')
}

export function closeSplashscreen(): Promise<void> {
  return invoke<void>('close_splashscreen')
}

export function restartApp(): Promise<void> {
  return invoke<void>('restart')
}

export function canAutoUpdate(): Promise<boolean> {
  return invoke<boolean>('can_auto_update')
}

// Per-event listen helpers for the backend events. Callers must invoke the
// matching subscribe command first (browse_types, subscribe_interfaces,
// subscribe_metrics); add/remove and theme events need no subscription.

export function onServiceTypeFound(
  cb: (payload: ServiceTypeFoundEvent) => void,
): Promise<UnlistenFn> {
  return listen<ServiceTypeFoundEvent>('service-type-found', (event) => cb(event.payload))
}

export function onServiceResolved(
  cb: (payload: ServiceResolvedEvent) => void,
): Promise<UnlistenFn> {
  return listen<ServiceResolvedEvent>('service-resolved', (event) => cb(event.payload))
}

export function onServiceRemoved(cb: (payload: ServiceRemovedEvent) => void): Promise<UnlistenFn> {
  return listen<ServiceRemovedEvent>('service-removed', (event) => cb(event.payload))
}

export function onInterfacesChanged(
  cb: (payload: InterfacesChangedEvent) => void,
): Promise<UnlistenFn> {
  return listen<InterfacesChangedEvent>('interfaces-changed', (event) => cb(event.payload))
}

export function onMetricsChanged(cb: (payload: MetricsChangedEvent) => void): Promise<UnlistenFn> {
  return listen<MetricsChangedEvent>('metrics-changed', (event) => cb(event.payload))
}

export function onThemeChanged(cb: (payload: ThemeChangedEvent) => void): Promise<UnlistenFn> {
  return listen<ThemeChangedEvent>('theme-changed', (event) => cb(event.payload))
}
