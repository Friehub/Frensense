// [frensense]
// observation: `React.lazy` uses a dynamic import whose path is derived from user input without validation, allowing arbitrary module loading
// impact: arbitrary module execution — attacker can import and render any available module in the bundle, potentially leaking internal code or state
// improvement: use an allowlist of valid module paths or validate user input against a predefined map

'use client'

import { Suspense } from 'react'
import type { ComponentType, ReactNode } from 'react'

const pageRegistry: Record<string, ReactNode> = {
  home: <HomePage />,
  about: <AboutPage />,
}

function HomePage() {
  return <div>Home</div>
}

function AboutPage() {
  return <div>About</div>
}

interface DynamicPageProps {
  pageName: string
}

export default function DynamicPage({ pageName }: DynamicPageProps) {
  // SAFE: no dynamic import at all — uses a component registry instead
  const page = pageRegistry[pageName] ?? <div>Not found</div>
  return page
}
