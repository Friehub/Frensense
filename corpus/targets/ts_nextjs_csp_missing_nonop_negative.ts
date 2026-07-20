// SAFE: CSP includes 'strict-dynamic' and a nonce, preventing script gadget attacks even if an inline script is injected

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          {
            key: 'Content-Security-Policy',
            value: "script-src 'self' 'strict-dynamic' 'nonce-{NONCE}'; object-src 'none'; base-uri 'none';",
          },
        ],
      },
    ]
  },
}

export default nextConfig
