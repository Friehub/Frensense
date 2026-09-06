// SAFE: multi-hop variables use default fallback
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const a = error?.code ?? 'N/A';
  const b = a;
  return (
    <div role="alert">
      <p>Error code: {b}</p>
      <p>{error?.message ?? 'Unknown error'}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
