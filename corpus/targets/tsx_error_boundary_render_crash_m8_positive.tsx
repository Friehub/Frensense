// [frensense]
// observation: Error property is accessed via array index to a non-array value, causing crash.
// impact: Fallback crash and infinite error loop.
// improvement: Guard against non-array access or use optional chaining.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {[error.code][0]}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
