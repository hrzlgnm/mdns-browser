import { describe, expect, it } from 'vitest'
import { dropLocalAndTrailingDot, dropTrailingDot, getOpenUrls } from './browse-utils'
import type { ResolvedService, ScopedAddr } from './types'

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
