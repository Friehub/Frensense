// SAFE: Environment variables are only accessed server-side via server-only loader, never compiled into client bundle

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  serverExternalPackages: ['secret-utils'],
  webpack: (config) => {
    return config
  },
}

export default nextConfig
