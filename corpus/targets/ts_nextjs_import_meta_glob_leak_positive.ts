// [frensense]
// observation: `import.meta.glob` is used with a relative pattern in a client component, causing all matched file paths to be bundled into the client-side JavaScript bundle.
// impact: Internal file paths, directory structure, and potentially sensitive filenames are exposed to end users in the browser, aiding reconnaissance.
// improvement: Use `import.meta.glob` only in server components, or ensure the glob pattern does not match sensitive files.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { ReactNode } from 'react'

const pages = import.meta.glob('/app/**/page.tsx')

export default function PageIndex() {
  const entries = Object.keys(pages) as string[]

  return (
    <ul>
      {entries.map(path => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  )
}
