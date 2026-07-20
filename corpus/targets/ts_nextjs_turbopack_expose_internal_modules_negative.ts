// SAFE: import.meta.glob is only used in server components; client components never receive the resolved paths

import { ReactNode } from 'react'

const modules = import.meta.glob('/app/**/page.tsx')

export default async function PageList() {
  const entries = Object.keys(modules)
  return (
    <ul>
      {entries.map((path) => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  )
}
