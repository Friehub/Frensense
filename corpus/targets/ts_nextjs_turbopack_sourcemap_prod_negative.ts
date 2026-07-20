// SAFE: `productionBrowserSourceMaps` is explicitly set to `false` (or omitted, which defaults to false)

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  turbopack: true,
  productionBrowserSourceMaps: false,
}

export default nextConfig
