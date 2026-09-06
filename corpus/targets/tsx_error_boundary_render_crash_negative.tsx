// SAFE: fallback component safely accesses error properties with optional chaining

'use client'

import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error: {error?.message ?? 'Unknown error'}</p>
    </div>
  )
}

function BuggyComponent() {
  throw new Error('Something went wrong')
}

export default function App() {
  return (
    <ErrorBoundary FallbackComponent={Fallback}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
