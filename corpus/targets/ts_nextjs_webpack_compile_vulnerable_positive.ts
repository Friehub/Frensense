// [frensense]
// observation: The `next.config.ts` file extends webpack configuration with a dangerous plugin that evaluates user-controlled expressions at build time, shipped via an npm dependency that accepts config from environment variables.
// impact: An attacker who can influence the CI/CD pipeline or environment variables can inject arbitrary code execution during the next build, leading to supply-chain compromise.
// improvement: Avoid using dynamic webpack plugins that accept runtime configuration. Pin plugin versions and validate any configuration source.

import type { NextConfig } from 'next'
import webpack from 'webpack'

const nextConfig: NextConfig = {
  webpack: (config) => {
    config.plugins?.push(
      new webpack.DefinePlugin({
        'process.env.BUILD_SECRET': JSON.stringify(process.env.SECRET_KEY ?? ''),
      })
    )
    return config
  },
}

export default nextConfig
