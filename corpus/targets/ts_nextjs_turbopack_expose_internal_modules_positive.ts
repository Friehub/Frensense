// [frensense]
// observation: With Turbopack enabled as the default bundler in Next.js 16.3, internal module paths from the server file system are leaked in the client bundle's module registry when using `import.meta.glob` or barrel exports in client components.
// impact: An attacker can inspect the client-side JavaScript to discover internal directory structure, server component file names, and potentially sensitive file paths that aid in reconnaissance.
// improvement: Audit client components for accidental `import.meta.glob` usage, and use the `'use client'` boundary carefully to avoid leaking server-side module paths.

import { ReactNode } from 'react'

const modules = import.meta.glob('/app/**/page.tsx')

export default function PageList() {
  const entries = Object.keys(modules)
  return (
    <ul>
      {entries.map((path) => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  )
}
