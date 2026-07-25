// [frensense]
// observation: global-error.tsx renders the full error message and stack trace on the error page
// impact: sensitive information (file paths, internal logic, db schema) exposed to end users
// improvement: only show a generic error message; log details server-side
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

'use client'

export default function GlobalError({ error }: { error: Error & { digest?: string } }) {
  return (
    <html>
      <body>
        <h1>Something went wrong</h1>
        <pre>{error.message}</pre>
        <pre>{error.stack}</pre>
      </body>
    </html>
  )
}
