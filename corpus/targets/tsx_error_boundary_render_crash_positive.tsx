// [frensense]
// observation: The error boundary's fallback component itself throws an error during rendering (e.g., accessing a missing property on the error object)
// impact: infinite error loop — React repeatedly tries to render the fallback, which keeps throwing, eventually causing a browser tab crash or process OOM
// improvement: ensure the fallback component is robust and does not assume properties exist on the error object

'use client'

import { ErrorBoundary } from 'react-error-boundary'

function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {error.code}</p>
      <p>{error.message}</p>
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
