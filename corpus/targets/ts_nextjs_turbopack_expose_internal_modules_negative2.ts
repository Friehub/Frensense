// SAFE: Client component uses a pre-computed list from the server instead of direct import.meta.glob access

'use client'

export default function PageList({ pagePaths }: { pagePaths: string[] }) {
  return (
    <ul>
      {pagePaths.map((path) => (
        <li key={path}>{path}</li>
      ))}
    </ul>
  )
}
