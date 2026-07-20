// SAFE: array access is guarded with optional chaining and default
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error?.code;
  return (
    <div role="alert">
      <p>Error code: {[code ?? 'N/A'][0]}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
