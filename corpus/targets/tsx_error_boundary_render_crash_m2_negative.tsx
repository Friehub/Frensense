// SAFE: intermediate variable uses optional chaining
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const code = error?.code ?? 'Unknown';
  return (
    <div role="alert">
      <p>Error code: {code}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
