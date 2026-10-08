import type { ResolvedService, ScopedAddr, TxtRecord } from 'tauri-plugin-mdns-api'

// Pure helpers ported from the former Leptos `src/app/browse.rs`,
// operating on the `tauri-plugin-mdns-api` wire types.

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

// TXT keys are case-insensitive (RFC 6763, section 6.4), so the details
// view orders them by case-folded key. Fold ties fall back to the raw key
// for a deterministic order. Returns a copy; the store keeps wire order.
export function sortTxtRecords(txt: Array<TxtRecord>): Array<TxtRecord> {
  return [...txt].sort(
    (a, b) =>
      compareStrings(a.key.toLowerCase(), b.key.toLowerCase()) || compareStrings(a.key, b.key),
  )
}

function isUnicastLinkLocalV6(ip: string): boolean {
  // fe80::/10 -> first hextet fe80..febf
  const first = ip.split(':')[0] ?? ''
  if (!/^[0-9a-fA-F]{1,4}$/.test(first)) return false
  const value = parseInt(first, 16)
  return (value & 0xffc0) === 0xfe80
}

function formatAddress(ip: string): string {
  return ip.includes(':') ? `[${ip}]` : ip
}

function normalizePath(rawPath: string | null | undefined): string {
  if (rawPath === null || rawPath === undefined) return '/'
  const trimmed = rawPath.trim()
  return trimmed.startsWith('/') ? trimmed : `/${trimmed}`
}

// Applies to the plain `_http._tcp.local.` / `_https._tcp.local.` services
// as well as `_sub` subtype services.
function httpScheme(serviceType: string): 'http' | 'https' | null {
  if (serviceType.includes('_https._tcp')) return 'https'
  if (serviceType.includes('_http._tcp')) return 'http'
  return null
}

function isHttpUrl(value: string): boolean {
  if (!/^https?:\/\//i.test(value)) return false
  try {
    new URL(value)
    return true
  } catch {
    return false
  }
}

// The address as a host string (scope stripped), or null when link-local v6.
function usableIp(address: ScopedAddr): string | null {
  const ip = addrIpString(address).split('%')[0]
  if (ip.includes(':') && isUnicastLinkLocalV6(ip)) return null
  return ip
}

// Gathers every URL a service can be opened with, in priority order: for
// http(s) services one per usable address (the first is the primary), then
// the hostname variant, then any http(s) TXT values. Deduplicated in
// insertion order; empty when the service is not openable.
export function getOpenUrls(service: ResolvedService): string[] {
  const urls = new Set<string>()

  const scheme = httpScheme(service.service_type)
  if (scheme !== null) {
    // TXT keys are case-insensitive (RFC 6763, section 6.4).
    const path = normalizePath(
      service.txt.find((record) => record.key.toLowerCase() === 'path')?.val,
    )
    for (const addr of service.addresses) {
      const ip = usableIp(addr)
      if (ip === null) continue
      urls.add(`${scheme}://${formatAddress(ip)}:${service.port}${path}`)
    }
    const hostname = dropTrailingDot(service.hostname)
    if (hostname !== '') {
      urls.add(`${scheme}://${hostname}:${service.port}${path}`)
    }
  }

  for (const record of service.txt) {
    const value = record.val
    if (value === null || value === undefined) continue
    const candidate = value.trim()
    if (isHttpUrl(candidate)) urls.add(candidate)
  }

  return [...urls]
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

// Parses dotted-decimal IPv4 into four numeric octets. Leading zeros are
// accepted and compared by value, so `192.168.0.002` equals `192.168.0.2`.
function parseIpv4(s: string): Array<number> | null {
  const parts = s.split('.')
  if (parts.length !== 4) return null
  const out: Array<number> = []
  for (const part of parts) {
    if (part === undefined || !/^\d{1,3}$/.test(part)) return null
    const n = parseInt(part, 10)
    if (n < 0 || n > 255) return null
    out.push(n)
  }
  return out
}

function parseHextets(text: string): Array<number> | null {
  if (text === '') return []
  const parts = text.split(':')
  const out: Array<number> = []
  for (const part of parts) {
    if (part === undefined || !/^[0-9a-fA-F]{1,4}$/.test(part)) return null
    out.push(parseInt(part, 16))
  }
  return out
}

// Parses an IPv6 literal into sixteen bytes in network order. Handles `::`
// compression and embedded IPv4 tails such as `::ffff:192.168.0.1`.
function parseIpv6(s: string): Array<number> | null {
  let text = s
  if (text.includes('.')) {
    const tail = text.slice(text.lastIndexOf(':') + 1)
    const v4 = parseIpv4(tail)
    if (v4 === null) return null
    const prefix = text.slice(0, text.length - tail.length)
    const high = ((v4[0] ?? 0) * 256 + (v4[1] ?? 0)).toString(16)
    const low = ((v4[2] ?? 0) * 256 + (v4[3] ?? 0)).toString(16)
    text = `${prefix}${high}:${low}`
  }
  const halves = text.split('::')
  if (halves.length > 2) return null
  let groups: Array<number>
  if (halves.length === 2) {
    const left = parseHextets(halves[0] ?? '')
    const right = parseHextets(halves[1] ?? '')
    if (left === null || right === null) return null
    const fill = 8 - (left.length + right.length)
    if (fill < 1) return null
    groups = [...left, ...new Array<number>(fill).fill(0), ...right]
  } else {
    const parsed = parseHextets(text)
    if (parsed === null || parsed.length !== 8) return null
    groups = parsed
  }
  const out: Array<number> = []
  for (const group of groups) {
    out.push((group >> 8) & 0xff, group & 0xff)
  }
  return out
}

// Numeric IP ordering with a lexicographic fallback for non-IP strings.
// IPv4 sorts before IPv6 so mixed families stay grouped deterministically.
function compareIpStrings(a: string, b: string): number {
  const sa = a.split('%')[0] ?? a
  const sb = b.split('%')[0] ?? b
  const a4 = parseIpv4(sa)
  const b4 = parseIpv4(sb)
  if (a4 !== null && b4 !== null) {
    for (let i = 0; i < 4; i++) {
      if (a4[i] !== b4[i]) return (a4[i] ?? 0) - (b4[i] ?? 0)
    }
    return 0
  }
  const a6 = parseIpv6(sa)
  const b6 = parseIpv6(sb)
  if (a6 !== null && b6 !== null) {
    for (let i = 0; i < 16; i++) {
      if (a6[i] !== b6[i]) return (a6[i] ?? 0) - (b6[i] ?? 0)
    }
    return 0
  }
  if (a4 !== null && b6 !== null) return -1
  if (a6 !== null && b4 !== null) return 1
  return compareStrings(a, b)
}

function compareScopedAddrSingle(x: ScopedAddr, y: ScopedAddr): number {
  if (x.addr !== y.addr) {
    const ipOrder = compareIpStrings(x.addr, y.addr)
    if (ipOrder !== 0) return ipOrder
  }
  const xi = x.interfaces.map((iface) => `${iface.name}#${iface.index}`).join(',')
  const yi = y.interfaces.map((iface) => `${iface.name}#${iface.index}`).join(',')
  if (xi !== yi) return compareStrings(xi, yi)
  const xs = x.scope_id ?? null
  const ys = y.scope_id ?? null
  if (xs === null && ys !== null) return -1
  if (xs !== null && ys === null) return 1
  if (xs !== null && ys !== null && xs !== ys) return compareStrings(xs, ys)
  return 0
}

// Numerically sorted copy of a service's addresses for display. The
// discovery order is preserved in the store.
export function sortAddresses(addresses: Array<ScopedAddr>): Array<ScopedAddr> {
  return [...addresses].sort(compareScopedAddrSingle)
}

function compareScopedAddrs(a: Array<ScopedAddr>, b: Array<ScopedAddr>): number {
  // Rank by the same numeric order the details view shows, so a service
  // holding `192.168.0.155` and `192.168.0.2` sorts under `.2`.
  const sa = sortAddresses(a)
  const sb = sortAddresses(b)
  const len = Math.min(sa.length, sb.length)
  for (let i = 0; i < len; i++) {
    const x = sa[i]
    const y = sb[i]
    if (x === undefined || y === undefined) break
    const order = compareScopedAddrSingle(x, y)
    if (order !== 0) return order
  }
  if (sa.length !== sb.length) return sa.length < sb.length ? -1 : 1
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

// Fullnames of instances marked dead. Records expire while the app is
// suspended (the querier's query loop stalls with the CPU), so callers
// restart instance browsing on foregrounding.
export function deadInstanceNames(services: Map<string, ResolvedService>): Array<string> {
  const names: Array<string> = []
  for (const service of services.values()) {
    if (service.dead) names.push(service.instance_fullname)
  }
  return names
}
