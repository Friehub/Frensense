// [frensense]
// observation: `React.lazy` uses a dynamic import whose path is derived from user input without validation, allowing arbitrary module loading
// impact: arbitrary module execution — attacker can import and render any available module in the bundle, potentially leaking internal code or state
// improvement: use an allowlist of valid module paths or validate user input against a predefined map

'use client'

import { lazy, Suspense } from 'react'
import type { ComponentType } from 'react'

const pageMap: Record<string, () => Promise<{ default: ComponentType<Record<string, never>> }>> = {
  home: () => import('./pages/home') as Promise<{ default: ComponentType<Record<string, never>> }>,
  about: () => import('./pages/about') as Promise<{ default: ComponentType<Record<string, never>> }>,
  contact: () => import('./pages/contact') as Promise<{ default: ComponentType<Record<string, never>> }>,
}

interface DynamicPageProps {
  pageName: string
}

export default function DynamicPage({ pageName }: DynamicPageProps) {
  const importFn = pageMap[pageName]
  if (!importFn) throw new Error('Invalid page')

  // SAFE: pageName is validated against an allowlist before lazy import
  const Page = lazy(importFn)

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <Page />
    </Suspense>
  )
}
