// SAFE: Source maps are generated locally for error tracking but not served in production by configuring a reverse proxy to block .map files

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  turbopack: true,
  productionBrowserSourceMaps: false,
  experimental: {
    serverSourceMaps: true,
  },
}

export default nextConfig
