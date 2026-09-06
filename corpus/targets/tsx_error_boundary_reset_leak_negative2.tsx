// SAFE: onReset only resets the key without logging — error logging is done server-side instead

'use client'

import { useState } from 'react'
import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ resetErrorBoundary }: { error: Error; resetErrorBoundary: () => void }) {
  return (
    <div role="alert">
      <p>An unexpected error occurred.</p>
      <button onClick={resetErrorBoundary}>Try again</button>
    </div>
  )
}

export default function App() {
  const [key, setKey] = useState(0)

  return (
    <ErrorBoundary
      FallbackComponent={Fallback}
      onReset={() => setKey((k) => k + 1)}
      onError={(error) => {
        void fetch('/api/log-error', { method: 'POST', body: JSON.stringify({ message: error.message }) })
      }}
      resetKeys={[key]}
    >
      <BuggyComponent />
    </ErrorBoundary>
  )
}

function BuggyComponent() {
  throw new Error('Database connection failed at host: internal-db-01.prod.example.com')
}
