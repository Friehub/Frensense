// SAFE: user input is sanitized before rendering in the fallback using textContent (React's default escaping)

'use client'

import { Suspense } from 'react'

export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {searchQuery}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}

async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query)
  const results = await fetch(`/api/search?q=${encoded}`).then((r) => r.json())
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>
}
