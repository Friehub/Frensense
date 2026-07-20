// [frensense]
// observation: Suspense fallback renders user-controlled content (e.g., search query, URL parameter) without sanitization
// impact: XSS before hydration — the fallback HTML is server-rendered with unescaped user input, allowing script injection before React hydrates
// improvement: sanitize or escape user input before rendering it in the fallback

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
  const results = await fetch(`/api/search?q=${query}`).then((r) => r.json())
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>
}
