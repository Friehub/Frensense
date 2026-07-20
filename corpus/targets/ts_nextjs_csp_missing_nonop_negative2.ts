// SAFE: CSP uses a hash-based policy for inline scripts with 'strict-dynamic' fallback

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  async headers() {
    return [
      {
        source: '/(.*)',
        headers: [
          {
            key: 'Content-Security-Policy',
            value: "script-src 'self' 'strict-dynamic' 'sha256-ABC123...'; object-src 'none';",
          },
        ],
      },
    ]
  },
}

export default nextConfig
