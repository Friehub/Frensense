// [frensense]
// observation: Error property is destructured before rendering, crashing when code does not exist.
// impact: Fallback crash — destructuring undefined causes TypeError.
// improvement: Provide default value in destructuring pattern.
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const { code } = error as any;
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
