// [frensense]
// observation: Error boundary wraps a component that throws an error asynchronously (inside setTimeout, Promise, or event handler) — the error boundary does not catch it
// impact: uncaught async errors crash the process (or cause unhandled rejections) with no fallback UI shown to the user
// improvement: catch async errors manually and call `reject` with the error, or use React's error boundary with `useErrorHandler` hook
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

'use client'

import { useState } from 'react'
import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ error, resetErrorBoundary }: { error: Error; resetErrorBoundary: () => void }) {
  return (
    <div role="alert">
      <p>Error: {error.message}</p>
      <button onClick={resetErrorBoundary}>Retry</button>
    </div>
  )
}

function AsyncBug() {
  const [loaded, setLoaded] = useState(false)

  if (!loaded) {
    setTimeout(() => {
      throw new Error('Async crash — caught by process, not error boundary')
    }, 100)
    setLoaded(true)
  }

  return <div>Loading complete</div>
}

export default function App() {
  return (
    <ErrorBoundary FallbackComponent={Fallback}>
      <AsyncBug />
    </ErrorBoundary>
  )
}
