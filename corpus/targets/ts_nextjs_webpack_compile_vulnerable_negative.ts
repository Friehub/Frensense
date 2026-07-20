// SAFE: Webpack configuration uses only compile-time constants and does not expose environment variables via DefinePlugin

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  webpack: (config) => {
    return config
  },
}

export default nextConfig
