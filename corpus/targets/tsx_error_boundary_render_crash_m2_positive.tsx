// [frensense]
// observation: The error object property is assigned to an intermediate variable that may not exist on the error.
// impact: Fallback component crashes when error.code is undefined, causing infinite error loop.
// improvement: Use optional chaining or default value for properties.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error.code;
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
