// [frensense]
// observation: Error property is rendered via template literal that throws when property is missing.
// impact: Fallback crash — error.code is undefined, template literal throws in strict mode.
// improvement: Use optional chaining in template expression.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium
'use client'
import { ErrorBoundary } from 'react-error-boundary'
function Fallback({ error }: { error: Error }) {
  return (
    <div role="alert">
      <p>Error code: {`${error.code}`}</p>
      <p>{error.message}</p>
    </div>
  )
}
function BuggyComponent() { throw new Error('Something went wrong') }
export default function App() {
  return <ErrorBoundary FallbackComponent={Fallback}><BuggyComponent /></ErrorBoundary>
}
