// SAFE: onReset does not log or expose the error — it simply resets the state

'use client'

import { useState } from 'react'
import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ error, resetErrorBoundary }: { error: Error; resetErrorBoundary: () => void }) {
  return (
    <div role="alert">
      <p>Something went wrong. Please try again.</p>
      <button onClick={resetErrorBoundary}>Try again</button>
    </div>
  )
}

export default function App() {
  const [key, setKey] = useState(0)

  function handleReset() {
    setKey((k) => k + 1)
  }

  return (
    <ErrorBoundary FallbackComponent={Fallback} onReset={handleReset} resetKeys={[key]}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}

function BuggyComponent() {
  throw new Error('Database connection failed at host: internal-db-01.prod.example.com')
}
