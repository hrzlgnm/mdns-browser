import { invoke } from '@tauri-apps/api/core'

// mDNS discovery commands, events, and types live in tauri-plugin-mdns
// now; re-export them here so frontend imports keep a single seam.
export {
  browseMany,
  browseTypes,
  getProtocolFlags,
  localNetworkStatus,
  onInterfacesChanged,
  onMetricsChanged,
  onServiceRemoved,
  onServiceResolved,
  onServiceTypeFound,
  requestLocalNetworkAccess,
  setInterfaces,
  setProtocolFlags,
  stopBrowse,
  subscribeInterfaces,
  subscribeMetrics,
  verifyInstance,
} from 'tauri-plugin-mdns-api'
export type {
  InterfacesChangedEvent,
  LocalNetworkState,
  MetricsChangedEvent,
  NetworkInterface,
  ProtocolFlags,
  ResolvedService,
  ScopedAddr,
  ServiceRemovedEvent,
  ServiceResolvedEvent,
  ServiceTypeFoundEvent,
  TxtRecord,
} from 'tauri-plugin-mdns-api'

// Mirrors the plugin's `ServiceTypes` Rust alias until
// tauri-plugin-mdns-api exports it.
export type ServiceTypes = Array<string>

// App commands still implemented by the backend itself.

export function getVersion(): Promise<string> {
  return invoke<string>('version')
}

export function isDesktop(): Promise<boolean> {
  return invoke<boolean>('is_desktop')
}

export function closeSplashscreen(): Promise<void> {
  return invoke<void>('close_splashscreen')
}

export function canAutoUpdate(): Promise<boolean> {
  return invoke<boolean>('can_auto_update')
}
