// SAFE: The remote pattern hostname is normalized with punycode encoding before matching, and wildcards are not used

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: 'cdn.example.com',
      },
      {
        protocol: 'https',
        hostname: 'images.example.com',
      },
    ],
  },
}

export default nextConfig
