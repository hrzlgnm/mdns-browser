import { describe, expect, it } from 'vitest'
import {
  compareServices,
  deadInstanceNames,
  dropLocalAndTrailingDot,
  dropTrailingDot,
  getOpenUrls,
  sortTxtRecords,
  sortAddresses,
} from './browse-utils'
import type { ResolvedService, ScopedAddr } from 'tauri-plugin-mdns-api'

function addr(ip: string, scope_id?: string): ScopedAddr {
  return { addr: ip, interfaces: [], scope_id: scope_id ?? null }
}

function service(overrides: Partial<ResolvedService> = {}): ResolvedService {
  return {
    instance_fullname: 'Test._http._tcp.local.',
    service_type: '_http._tcp.local.',
    hostname: 'test.local.',
    port: 8080,
    addresses: [],
    subtype: null,
    txt: [],
    updated_at_micros: '0',
    dead: false,
    ...overrides,
  }
}

describe('dropTrailingDot', () => {
  it('strips a single trailing dot', () => {
    expect(dropTrailingDot('test.local.')).toBe('test.local')
  })

  it('leaves names without a trailing dot untouched', () => {
    expect(dropTrailingDot('test.local')).toBe('test.local')
  })
})

describe('dropLocalAndTrailingDot', () => {
  it('strips .local. suffix and trailing dot', () => {
    expect(dropLocalAndTrailingDot('_http._tcp.local.')).toBe('_http._tcp')
  })

  it('leaves other suffixes untouched apart from the trailing dot', () => {
    expect(dropLocalAndTrailingDot('example.com.')).toBe('example.com')
  })
})

describe('getOpenUrls', () => {
  it('returns an IP URL and a hostname URL for a single IPv4 address', () => {
    expect(getOpenUrls(service({ addresses: [addr('192.168.1.10')] }))).toEqual([
      'http://192.168.1.10:8080/',
      'http://test.local:8080/',
    ])
  })

  it('uses https for _https services', () => {
    expect(
      getOpenUrls(
        service({
          service_type: '_https._tcp.local.',
          addresses: [addr('192.168.1.10')],
        }),
      ),
    ).toEqual(['https://192.168.1.10:8080/', 'https://test.local:8080/'])
  })

  it('detects the scheme in subtype service types', () => {
    expect(
      getOpenUrls(
        service({
          service_type: '_printer._sub._http._tcp.local.',
          addresses: [addr('192.168.1.10')],
        }),
      ),
    ).toEqual(['http://192.168.1.10:8080/', 'http://test.local:8080/'])
  })

  it('orders multiple addresses before the hostname and brackets IPv6', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('192.168.1.10'), addr('2001:db8::1')],
        }),
      ),
    ).toEqual([
      'http://192.168.1.10:8080/',
      'http://[2001:db8::1]:8080/',
      'http://test.local:8080/',
    ])
  })

  it('skips link-local IPv6 addresses but keeps the hostname URL', () => {
    expect(getOpenUrls(service({ addresses: [addr('fe80::1')] }))).toEqual([
      'http://test.local:8080/',
    ])
  })

  it('skips link-local IPv6 addresses alongside usable ones', () => {
    expect(getOpenUrls(service({ addresses: [addr('fe80::1'), addr('192.168.1.10')] }))).toEqual([
      'http://192.168.1.10:8080/',
      'http://test.local:8080/',
    ])
  })

  it('treats the full fe80::/10 range as link-local', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('fe80::1'), addr('febf::1234'), addr('192.168.1.10')],
        }),
      ),
    ).toEqual(['http://192.168.1.10:8080/', 'http://test.local:8080/'])
  })

  it('keeps non-link-local fe-prefixed IPv6 addresses', () => {
    expect(getOpenUrls(service({ addresses: [addr('fec0::1'), addr('fe7f::1')] }))).toEqual([
      'http://[fec0::1]:8080/',
      'http://[fe7f::1]:8080/',
      'http://test.local:8080/',
    ])
  })

  it('strips the scope id from scoped addresses', () => {
    expect(getOpenUrls(service({ addresses: [addr('192.168.1.10', '3')] }))).toEqual([
      'http://192.168.1.10:8080/',
      'http://test.local:8080/',
    ])
  })

  it('applies the TXT path to IP and hostname URLs', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('192.168.1.10')],
          txt: [{ key: 'path', val: 'index.html' }],
        }),
      ),
    ).toEqual(['http://192.168.1.10:8080/index.html', 'http://test.local:8080/index.html'])
  })

  it('matches the TXT path key case-insensitively', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('192.168.1.10')],
          txt: [{ key: 'Path', val: 'status' }],
        }),
      ),
    ).toEqual(['http://192.168.1.10:8080/status', 'http://test.local:8080/status'])
  })

  it('appends http(s) TXT values after the hostname URL', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('192.168.1.10')],
          txt: [
            { key: 'path', val: null },
            { key: 'url', val: 'https://example.com/status' },
            { key: 'note', val: 'not a url' },
          ],
        }),
      ),
    ).toEqual([
      'http://192.168.1.10:8080/',
      'http://test.local:8080/',
      'https://example.com/status',
    ])
  })

  it('deduplicates a TXT URL identical to the hostname URL', () => {
    expect(
      getOpenUrls(
        service({
          addresses: [addr('192.168.1.10')],
          txt: [{ key: 'url', val: 'http://test.local:8080/' }],
        }),
      ),
    ).toEqual(['http://192.168.1.10:8080/', 'http://test.local:8080/'])
  })

  it('returns an empty list for non-http services without URL TXT records', () => {
    expect(
      getOpenUrls(
        service({
          service_type: '_ssh._tcp.local.',
          addresses: [addr('192.168.1.10')],
        }),
      ),
    ).toEqual([])
  })

  it('returns only TXT URLs for non-http services', () => {
    expect(
      getOpenUrls(
        service({
          service_type: '_ssh._tcp.local.',
          addresses: [addr('192.168.1.10')],
          txt: [{ key: 'url', val: 'http://example.com/console' }],
        }),
      ),
    ).toEqual(['http://example.com/console'])
  })

  it('returns an empty list for UDP services', () => {
    expect(
      getOpenUrls(
        service({
          service_type: '_dns._udp.local.',
          addresses: [addr('192.168.1.10')],
        }),
      ),
    ).toEqual([])
  })

  it('omits the hostname URL when the hostname is empty', () => {
    expect(getOpenUrls(service({ hostname: '', addresses: [addr('192.168.1.10')] }))).toEqual([
      'http://192.168.1.10:8080/',
    ])
  })

  it('keeps a hostname without trailing dot as-is', () => {
    expect(
      getOpenUrls(service({ hostname: 'test.local', addresses: [addr('192.168.1.10')] })),
    ).toEqual(['http://192.168.1.10:8080/', 'http://test.local:8080/'])
  })

  it('reflects a custom port in every URL', () => {
    expect(getOpenUrls(service({ port: 8443, addresses: [addr('192.168.1.10')] }))).toEqual([
      'http://192.168.1.10:8443/',
      'http://test.local:8443/',
    ])
  })
})

describe('deadInstanceNames', () => {
  it('returns an empty list when no services are dead', () => {
    const services = new Map([
      ['A._http._tcp.local.', service({ instance_fullname: 'A._http._tcp.local.' })],
      ['B._http._tcp.local.', service({ instance_fullname: 'B._http._tcp.local.' })],
    ])
    expect(deadInstanceNames(services)).toEqual([])
  })

  it('returns only the dead instance names', () => {
    const services = new Map([
      ['A._http._tcp.local.', service({ instance_fullname: 'A._http._tcp.local.', dead: true })],
      ['B._http._tcp.local.', service({ instance_fullname: 'B._http._tcp.local.' })],
      // The wire fullname is queried, never the store key.
      ['stale-key', service({ instance_fullname: 'C._http._tcp.local.', dead: true })],
    ])
    expect(deadInstanceNames(services)).toEqual(['A._http._tcp.local.', 'C._http._tcp.local.'])
  })

  it('returns an empty list for an empty store', () => {
    expect(deadInstanceNames(new Map())).toEqual([])
  })
})

describe('sortTxtRecords', () => {
  it('orders keys case-insensitively (RFC 6763, section 6.4)', () => {
    const txt = [
      { key: 'banana', val: null },
      { key: 'Apple', val: null },
      { key: 'cherry', val: '1' },
    ]
    expect(sortTxtRecords(txt).map((record) => record.key)).toEqual(['Apple', 'banana', 'cherry'])
  })

  it('breaks case-fold ties by raw key without mutating the input', () => {
    const txt = [
      { key: 'b', val: null },
      { key: 'B', val: null },
    ]
    expect(sortTxtRecords(txt).map((record) => record.key)).toEqual(['B', 'b'])
    expect(txt.map((record) => record.key)).toEqual(['b', 'B'])
  })
})

describe('compareServices IpAddr', () => {
  function serviceWithIp(ip: string): ResolvedService {
    return service({ addresses: [addr(ip)] })
  }

  function sortedIps(ips: Array<string>, sort: 'IpAddrAsc' | 'IpAddrDesc'): Array<string> {
    return ips
      .map((ip) => serviceWithIp(ip))
      .sort((a, b) => compareServices(a, b, sort))
      .map((s) => s.addresses[0]?.addr ?? '')
  }

  it('sorts IPv4 numerically, not lexicographically (#2868)', () => {
    const shuffled = [
      '192.168.0.155',
      '192.168.0.163',
      '192.168.0.2',
      '192.168.0.200',
      '192.168.0.7',
      '192.168.0.77',
    ]
    expect(sortedIps(shuffled, 'IpAddrAsc')).toEqual([
      '192.168.0.2',
      '192.168.0.7',
      '192.168.0.77',
      '192.168.0.155',
      '192.168.0.163',
      '192.168.0.200',
    ])
  })

  it('reverses IPv4 order for descending sort', () => {
    const shuffled = ['192.168.0.7', '192.168.0.2', '192.168.0.155']
    expect(sortedIps(shuffled, 'IpAddrDesc')).toEqual([
      '192.168.0.155',
      '192.168.0.7',
      '192.168.0.2',
    ])
  })

  it('treats leading zeros by numeric value', () => {
    expect(
      compareServices(serviceWithIp('192.168.0.002'), serviceWithIp('192.168.0.2'), 'IpAddrAsc'),
    ).toBe(0)
    expect(sortedIps(['10.0.0.10', '10.0.0.9'], 'IpAddrAsc')).toEqual(['10.0.0.9', '10.0.0.10'])
  })

  it('sorts IPv6 numerically', () => {
    expect(sortedIps(['2001:db8::10', '2001:db8::1', '2001:db8::2'], 'IpAddrAsc')).toEqual([
      '2001:db8::1',
      '2001:db8::2',
      '2001:db8::10',
    ])
    expect(sortedIps(['::1', '2001:db8::1'], 'IpAddrAsc')).toEqual(['::1', '2001:db8::1'])
  })

  it('sorts IPv4 before IPv6', () => {
    expect(sortedIps(['2001:db8::1', '192.168.0.2'], 'IpAddrAsc')).toEqual([
      '192.168.0.2',
      '2001:db8::1',
    ])
  })

  it('ranks multi-address services by their sorted addresses', () => {
    const a = service({ addresses: [addr('192.168.0.155'), addr('192.168.0.2')] })
    const b = service({ addresses: [addr('192.168.0.7')] })
    expect([a, b].sort((x, y) => compareServices(x, y, 'IpAddrAsc'))).toEqual([a, b])
    expect([a, b].sort((x, y) => compareServices(x, y, 'IpAddrDesc'))).toEqual([b, a])
  })
})

describe('sortAddresses', () => {
  it('returns addresses in numeric order without mutating the input', () => {
    const input = [addr('192.168.0.155'), addr('192.168.0.2'), addr('192.168.0.77')]
    const sorted = sortAddresses(input).map((a) => a.addr)
    expect(sorted).toEqual(['192.168.0.2', '192.168.0.77', '192.168.0.155'])
    expect(input.map((a) => a.addr)).toEqual(['192.168.0.155', '192.168.0.2', '192.168.0.77'])
  })

  it('keeps scope tiebreakers for equal IPs', () => {
    const sorted = sortAddresses([addr('192.168.0.2', '3'), addr('192.168.0.2')])
    expect(sorted.map((a) => a.scope_id)).toEqual([null, '3'])
  })
})
