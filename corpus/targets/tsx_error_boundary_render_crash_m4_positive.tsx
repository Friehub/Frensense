// [frensense]
// observation: Error property is retrieved via a helper function that may return undefined.
// impact: Fallback crash when error.code is missing, causing infinite error loop.
// improvement: Ensure helper returns a safe default value.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function getCode(e: Error): any { return (e as any).code; }
function Fallback({ error }: { error: Error }) {
  const code = getCode(error);
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
