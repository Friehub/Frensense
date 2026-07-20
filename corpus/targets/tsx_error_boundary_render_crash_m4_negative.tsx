// SAFE: helper returns a safe default
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function getCode(e: Error): string { return (e as any).code ?? 'N/A'; }
function Fallback({ error }: { error: Error }) {
  const code = getCode(error);
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
