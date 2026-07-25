// [frensense]
// observation: The `onReset` callback of an error boundary exposes the previous error's message and stack trace to user-facing UI or logs it to the console
// impact: sensitive internal error details (stack traces, SQL queries, file paths) are leaked to users or observable logs
// improvement: sanitize error details in the onReset callback — log them for diagnostics but do not expose to user
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
      <p>Something went wrong:</p>
      <pre>{error.message}</pre>
      <button onClick={resetErrorBoundary}>Try again</button>
    </div>
  )
}

export default function App() {
  const [key, setKey] = useState(0)

  function handleReset(details: { error: Error }) {
    console.error('Previous error details:', details.error.stack)
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
