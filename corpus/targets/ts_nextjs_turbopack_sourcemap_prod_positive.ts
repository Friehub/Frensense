// [frensense]
// observation: The Turbopack build configuration in next.config.ts enables `productionBrowserSourceMaps: true`, which causes source map files to be emitted alongside production JavaScript bundles.
// impact: Full application source code (including server-side logic, API keys in comments, and internal logic) is exposed to anyone who inspects the /_next/static/chunks/ directory in the browser.
// improvement: Set `productionBrowserSourceMaps: false` (the default) and ensure source maps are never published to the production CDN.

import type { NextConfig } from 'next'

const nextConfig: NextConfig = {
  turbopack: true,
  productionBrowserSourceMaps: true,
}

export default nextConfig
