// SAFE: Remote patterns use an exact hostname allowlist with no wildcard, preventing unicode bypass via wildcard expansion

import type { NextConfig } from 'next'

const ALLOWED_HOSTNAMES = [
  'cdn.example.com',
  'images.example.com',
  'media.example.com',
]

const nextConfig: NextConfig = {
  images: {
    remotePatterns: ALLOWED_HOSTNAMES.map((hostname) => ({
      protocol: 'https' as const,
      hostname,
    })),
  },
}

export default nextConfig
