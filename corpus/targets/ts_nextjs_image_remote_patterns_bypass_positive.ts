// [frensense]
// observation: The `next.config.js` `remotePatterns` uses a regex that can be bypassed via unicode normalization or URL encoding tricks. The pattern only matches lowercase ASCII hostnames but the runtime normalizes unicode characters before matching.
// impact: An attacker can host malicious images on a domain that bypasses the pattern check (e.g. `evіl.com` using a Cyrillic `і` instead of ASCII `i`), enabling SSRF or hosting malicious content.
// improvement: Use an exact hostname allowlist instead of regex patterns, or normalize to punycode before matching.

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: '**.example.com',
      },
    ],
  },
}

export default nextConfig
