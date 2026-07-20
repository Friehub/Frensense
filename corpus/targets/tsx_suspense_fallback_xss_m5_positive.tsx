// [frensense]
// observation: User input is injected into Suspense fallback via template literal.
// impact: XSS before hydration.
// improvement: Escape template output or use generic message.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {`${searchQuery}`}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
