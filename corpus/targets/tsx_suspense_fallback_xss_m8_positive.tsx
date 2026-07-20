// [frensense]
// observation: User input is accessed via array index before rendering in Suspense fallback.
// impact: XSS before hydration — array element unsanitized.
// improvement: Escape or sanitize before fallback rendering.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string[] }) {
  return (
    <div>
      <Suspense fallback={<div>Searching for: {searchQuery[0]}</div>}>
        <SearchResults query={searchQuery[0]} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
