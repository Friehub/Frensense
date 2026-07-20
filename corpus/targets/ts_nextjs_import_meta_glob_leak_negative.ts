// SAFE: `import.meta.glob` is used in a server component only; the resolved paths never reach the client bundle

import { ReactNode } from 'react'

const pages = import.meta.glob('/app/**/page.tsx')

export default async function PageIndex() {
  const entries = Object.entries(pages) as [string, () => Promise<unknown>][]

  return (
    <ul>
      {entries.map(([path]) => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  )
}
