// [frensense]
// observation: The Content-Security-Policy header in next.config.js uses `script-src 'self'` without `'strict-dynamic'`, allowing an attacker who injects a single inline script to load arbitrary external JavaScript via DOM APIs.
// impact: An XSS vulnerability in any component bypasses the CSP because legacy bypasses (JSONP endpoints, script gadgets) remain exploitable without strict-dynamic enforcement.
// improvement: Add `'strict-dynamic'` with a nonce-based or hash-based CSP policy alongside `'self'` for production deployments.

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          {
            key: 'Content-Security-Policy',
            value: "script-src 'self' 'unsafe-inline'; object-src 'none';",
          },
        ],
      },
    ]
  },
}

export default nextConfig
