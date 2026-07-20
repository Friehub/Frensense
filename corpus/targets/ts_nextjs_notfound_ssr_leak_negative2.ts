// SAFE: A global error boundary catches all rendering errors and returns a generic 404 page without stack traces or source code paths
// CVE: CVE-2025-55183

'use client'

import { ReactNode } from 'react'

export default function GlobalError({ error, reset }: { error: Error; reset: () => void }) {
  console.error('Unhandled error in SSR:', error)
  return (
    <html>
      <body>
        <h1>Something went wrong</h1>
        <p>The page you requested could not be loaded.</p>
      </body>
    </html>
  )
}
