// [frensense]
// observation: `permanentRedirect` is called with a user-controlled URL from search parameters, and the response is cached by a CDN or the browser permanently via a 308 redirect.
// impact: An attacker can poison the permanent redirect cache, causing all subsequent visitors to be redirected to a malicious site until the cache expires or is purged.
// improvement: Validate that the target URL is relative or on an allowlist, and avoid using `permanentRedirect` with user input; use `redirect` instead with appropriate cache headers.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

import { permanentRedirect } from 'next/navigation'

export async function GET(request: Request) {
  const { searchParams } = new URL(request.url)
  const target = searchParams.get('dest')
  permanentRedirect(target ?? '/')
}
