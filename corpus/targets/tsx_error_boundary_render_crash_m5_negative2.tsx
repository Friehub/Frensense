// SAFE: fallbackRender prop handles errors safely
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return (
    <ErrorBoundary fallbackRender={({ error }) => (
      <div role="alert"><p>An error occurred: {error instanceof Error ? error.message : 'Unknown'}</p></div>
    )}>
      <BuggyComponent />
    </ErrorBoundary>
  )
}
