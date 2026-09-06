// SAFE: destructuring uses default value
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const { code = 'N/A', message = 'Unknown error' } = error as any;
  return (
    <div role="alert">
      <p>Error code: {code}</p>
      <p>{message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
