// [frensense]
// observation: Error property is concatenated with a string prefix, crashing when property is missing.
// impact: Fallback crash and infinite error loop.
// improvement: Guard with optional chaining before concatenation.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {"" + error.code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
