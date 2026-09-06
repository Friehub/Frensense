// [frensense]
// observation: Error property flows through two assignments before rendering, crashing on missing property.
// impact: Infinite error loop in fallback rendering.
// improvement: Guard against missing properties with fallback values.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  const a = error.code;
  const b = a;
  return (
    <div role="alert">
      <p>Error code: {b}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
