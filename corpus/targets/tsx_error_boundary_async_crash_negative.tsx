// SAFE: async errors are caught and re-thrown during render so the error boundary can handle them

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
  const [error, setError] = useState<Error | null>(null)

  if (!error) {
    setTimeout(() => {
      setError(new Error('Async crash caught by error boundary'))
    }, 100)
  }

  if (error) throw error

  return <div>Loading complete</div>
}

export default function App() {
  return (
    <ErrorBoundary FallbackComponent={Fallback}>
      <AsyncBug />
    </ErrorBoundary>
  )
}
