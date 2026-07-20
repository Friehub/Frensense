// [frensense]
// observation: `React.lazy` uses a dynamic import whose path is derived from user input without validation, allowing arbitrary module loading
// impact: arbitrary module execution — attacker can import and render any available module in the bundle, potentially leaking internal code or state
// improvement: use an allowlist of valid module paths or validate user input against a predefined map

'use client'

import { lazy, Suspense } from 'react'
import type { ComponentType } from 'react'

interface DynamicPageProps {
  pageName: string
}

export default function DynamicPage({ pageName }: DynamicPageProps) {
  const Page = lazy(() => import(`./pages/${pageName}`) as Promise<{ default: ComponentType<Record<string, never>> }>)

  return (
    <Suspense fallback={<div>Loading...</div>}>
      <Page />
    </Suspense>
  )
}
