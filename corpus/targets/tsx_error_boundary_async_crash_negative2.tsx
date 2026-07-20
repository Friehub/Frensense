// SAFE: uses async/await with error handling inside the component, re-throwing in render

'use client'

import { useEffect, useState } from 'react'
import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ error, resetErrorBoundary }: { error: Error; resetErrorBoundary: () => void }) {
  return (
    <div role="alert">
      <p>Error: {error.message}</p>
      <button onClick={resetErrorBoundary}>Retry</button>
    </div>
  )
}

function AsyncSafe() {
  const [state, setState] = useState<'loading' | 'done' | 'error'>('loading')

  useEffect(() => {
    let cancelled = false
    fetch('/api/data')
      .then((r) => {
        if (!r.ok) throw new Error('Async fetch failed')
        return r.json()
      })
      .then(() => { if (!cancelled) setState('done') })
      .catch((e) => { if (!cancelled) setState('error') })
    return () => { cancelled = true }
  }, [])

  if (state === 'error') throw new Error('Async fetch failed — caught by boundary')
  if (state === 'loading') return <div>Loading...</div>
  return <div>Data loaded</div>
}

export default function App() {
  return (
    <ErrorBoundary FallbackComponent={Fallback}>
      <AsyncSafe />
    </ErrorBoundary>
  )
}
