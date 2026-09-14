import type { ResolvedService, ScopedAddr, TxtRecord } from './types'

// Pure helpers ported from the former Leptos `src/app/browse.rs` and the
// `matches_query` / display impls in `crates/models`.

export type SortKind =
  | 'InstanceAsc'
  | 'InstanceDesc'
  | 'HostnameAsc'
  | 'HostnameDesc'
  | 'PortAsc'
  | 'PortDesc'
  | 'ServiceTypeAsc'
  | 'ServiceTypeDesc'
  | 'IpAddrAsc'
  | 'IpAddrDesc'
  | 'TimestampAsc'
  | 'TimestampDesc'

export function dropTrailingDot(fqn: string): string {
  return fqn.endsWith('.') ? fqn.slice(0, -1) : fqn
}

export function dropLocalAndTrailingDot(fqn: string): string {
  const withoutLocal = fqn.endsWith('.local.') ? fqn.slice(0, -'.local.'.length) : fqn
  return dropTrailingDot(withoutLocal)
}

export function isSubsequence(searchTerm: string, target: string): boolean {
  let i = 0
  for (const c of target) {
    if (i < searchTerm.length && searchTerm[i] === c) i++
  }
  return i === searchTerm.length
}

export function getPrefix(s: string): string {
  if (s.startsWith('_')) return s.slice(1)
  return s.split('.')[0] ?? s
}

function checkMdnsLabel(label: string): boolean {
  if (!label.startsWith('_')) return false
  const content = label.slice(1)
  if (content.startsWith('_') || content.endsWith('_')) return false
  if (![...content].every((c) => /[A-Za-z0-9_-]/.test(c))) return false
  if (content.includes('--')) return false
  if (content.startsWith('-') || content.endsWith('-')) return false
  return true
}

export function isValidServiceType(serviceType: string): boolean {
  if (!serviceType.endsWith('.')) return false
  const parts = serviceType.slice(0, -1).split('.')
  if (parts.length !== 3 && parts.length !== 5) return false
  const protocol = parts[parts.length - 2]
  if (protocol !== '_tcp' && protocol !== '_udp') return false
  if (parts[parts.length - 1] !== 'local') return false
  const service = parts.length === 3 ? parts[0] : parts[2]
  if (service === undefined || !checkMdnsLabel(service)) return false
  if (parts.length === 5) {
    if (parts[1] !== '_sub') return false
    const subType = parts[0]
    if (subType === undefined || !checkMdnsLabel(subType)) return false
  }
  return true
}

function genericJaro(a: Array<string>, b: Array<string>): number {
  if (a.length === 0 && b.length === 0) return 1
  if (a.length === 0 || b.length === 0) return 0
  // Integer match window like strsim: (max_len / 2).saturating_sub(1).
  const range = Math.max(0, Math.floor(Math.max(a.length, b.length) / 2) - 1)
  const aMatches = new Array<boolean>(a.length).fill(false)
  const bMatches = new Array<boolean>(b.length).fill(false)
  let matches = 0
  for (let i = 0; i < a.length; i++) {
    const start = Math.max(0, i - range)
    const end = Math.min(b.length, i + range + 1)
    for (let j = start; j < end; j++) {
      if (!bMatches[j] && a[i] === b[j]) {
        aMatches[i] = true
        bMatches[j] = true
        matches++
        break
      }
    }
  }
  if (matches === 0) return 0
  let transpositions = 0
  let j = 0
  for (let i = 0; i < a.length; i++) {
    if (aMatches[i]) {
      while (!bMatches[j]) j++
      if (a[i] !== b[j]) transpositions++
      j++
    }
  }
  transpositions /= 2
  return (matches / a.length + matches / b.length + (matches - transpositions) / matches) / 3
}

// Mirrors `strsim::jaro_winkler` (threshold 0.7, 0.1 weight, 4-char prefix).
export function jaroWinkler(a: string, b: string): number {
  const charsA = [...a]
  const charsB = [...b]
  const jaro = genericJaro(charsA, charsB)
  if (jaro > 0.7) {
    let prefix = 0
    while (prefix < 4 && prefix < charsA.length && charsA[prefix] === charsB[prefix]) prefix++
    return jaro + 0.1 * prefix * (1 - jaro)
  }
  return jaro
}

export function serviceTypeMatches(input: string, candidate: string): boolean {
  const lookup = getPrefix(input)
  const prefix = getPrefix(candidate.split('.')[0] ?? candidate)
  return jaroWinkler(lookup, prefix) >= 0.75 || isSubsequence(lookup, prefix)
}

export function addrDisplay(addr: ScopedAddr): string {
  if (addr.scope_id !== undefined && addr.scope_id !== null) {
    return `${addr.addr}%${addr.scope_id}`
  }
  if (addr.interfaces.length === 0) return addr.addr
  return `${addr.addr} via ${addr.interfaces.map((i) => i.name).join(', ')}`
}

export function addrIpString(addr: ScopedAddr): string {
  if (addr.scope_id !== undefined && addr.scope_id !== null) {
    return `${addr.addr}%${addr.scope_id}`
  }
  return addr.addr
}

export function txtDisplay(record: TxtRecord): string {
  if (record.val === null || record.val === undefined) return record.key
  return `${record.key}=${record.val}`
}

function isUnicastLinkLocalV6(ip: string): boolean {
  // fe80::/10 -> first hextet fe80..febf
  const first = ip.split(':')[0] ?? ''
  if (!/^[0-9a-fA-F]{1,4}$/.test(first)) return false
  const value = parseInt(first, 16)
  return (value & 0xffc0) === 0xfe80
}

function firstUsableAddress(service: ResolvedService): string | null {
  for (const addr of service.addresses) {
    const ip = addrIpString(addr).split('%')[0] ?? ''
    if (!ip.includes(':')) return ip
    if (!isUnicastLinkLocalV6(ip)) return ip
  }
  return null
}

function formatAddress(ip: string): string {
  return ip.includes(':') ? `[${ip}]` : ip
}

export function getOpenUrl(service: ResolvedService): string | null {
  const rawPath = service.txt.find((record) => record.key === 'path')?.val
  const path =
    rawPath === null || rawPath === undefined
      ? null
      : rawPath.startsWith('/')
        ? rawPath
        : `/${rawPath}`
  const address = firstUsableAddress(service)
  if (address === null) return null
  const internalUrl = service.txt.find((record) => record.key === 'internal_url')?.val
  switch (service.service_type) {
    case '_http._tcp.local.':
      return `http://${formatAddress(address)}:${service.port}${path ?? '/'}`
    case '_https._tcp.local.':
      return `https://${formatAddress(address)}:${service.port}${path ?? '/'}`
    case '_home-assistant._tcp.local.':
      return internalUrl === null || internalUrl === undefined ? null : internalUrl
    default:
      return null
  }
}

export function matchesQuery(service: ResolvedService, rawQuery: string): boolean {
  const query = rawQuery.toLowerCase()
  if (query === '') return true
  if (service.instance_fullname.toLowerCase().includes(query)) return true
  if (service.service_type.toLowerCase().includes(query)) return true
  if (service.hostname.toLowerCase().includes(query)) return true
  if (service.port.toString().includes(query)) return true
  if (service.addresses.some((addr) => addrDisplay(addr).includes(query))) return true
  if (
    service.subtype !== null &&
    service.subtype !== undefined &&
    service.subtype.toLowerCase().includes(query)
  ) {
    return true
  }
  if (service.txt.some((record) => txtDisplay(record).toLowerCase().includes(query))) return true
  if (query === 'dead' && service.dead) return true
  if (query === 'alive' && !service.dead) return true
  return false
}

// Microsecond timestamps arrive as strings on the wire; format like
// `%Y-%m-%d %H:%M:%S%.6f` in local time.
export function toLocalTimestamp(timestampMicros: string): string {
  try {
    const micros = BigInt(timestampMicros)
    const date = new Date(Number(micros / 1000n))
    if (Number.isNaN(date.getTime())) return 'Invalid timestamp'
    const pad = (n: number, len = 2) => String(n).padStart(len, '0')
    const fraction = String(micros % 1000000n).padStart(6, '0')
    return (
      `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())} ` +
      `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}.${fraction}`
    )
  } catch {
    return 'Invalid timestamp'
  }
}

function compareStrings(a: string, b: string): number {
  return a < b ? -1 : a > b ? 1 : 0
}

function compareMicros(a: string, b: string): number {
  if (a.length !== b.length) return a.length < b.length ? -1 : 1
  return compareStrings(a, b)
}

function compareScopedAddrs(a: Array<ScopedAddr>, b: Array<ScopedAddr>): number {
  const len = Math.min(a.length, b.length)
  for (let i = 0; i < len; i++) {
    const x = a[i]
    const y = b[i]
    if (x === undefined || y === undefined) break
    if (x.addr !== y.addr) return compareStrings(x.addr, y.addr)
    const xi = x.interfaces.map((iface) => `${iface.name}#${iface.index}`).join(',')
    const yi = y.interfaces.map((iface) => `${iface.name}#${iface.index}`).join(',')
    if (xi !== yi) return compareStrings(xi, yi)
    const xs = x.scope_id ?? null
    const ys = y.scope_id ?? null
    if (xs === null && ys !== null) return -1
    if (xs !== null && ys === null) return 1
    if (xs !== null && ys !== null && xs !== ys) return compareStrings(xs, ys)
  }
  if (a.length !== b.length) return a.length < b.length ? -1 : 1
  return 0
}

export function compareServices(a: ResolvedService, b: ResolvedService, sort: SortKind): number {
  switch (sort) {
    case 'InstanceAsc':
      return compareStrings(a.instance_fullname, b.instance_fullname)
    case 'InstanceDesc':
      return compareStrings(b.instance_fullname, a.instance_fullname)
    case 'HostnameAsc':
      return (
        compareStrings(a.hostname, b.hostname) || compareStrings(a.service_type, b.service_type)
      )
    case 'HostnameDesc':
      return (
        compareStrings(b.hostname, a.hostname) || compareStrings(b.service_type, a.service_type)
      )
    case 'PortAsc':
      return a.port - b.port
    case 'PortDesc':
      return b.port - a.port
    case 'ServiceTypeAsc':
      return compareStrings(a.service_type, b.service_type)
    case 'ServiceTypeDesc':
      return compareStrings(b.service_type, a.service_type)
    case 'IpAddrAsc':
      return compareScopedAddrs(a.addresses, b.addresses)
    case 'IpAddrDesc':
      return compareScopedAddrs(b.addresses, a.addresses)
    case 'TimestampAsc':
      return compareMicros(a.updated_at_micros, b.updated_at_micros)
    case 'TimestampDesc':
      return compareMicros(b.updated_at_micros, a.updated_at_micros)
  }
}

export function getInstanceName(service: ResolvedService): string {
  const suffix = service.service_type
  let name = service.instance_fullname.endsWith(suffix)
    ? service.instance_fullname.slice(0, -suffix.length)
    : service.instance_fullname
  if (name.endsWith('.')) name = name.slice(0, -1)
  return name
}
