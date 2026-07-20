// [frensense]
// observation: User input is destructured before rendering in Suspense fallback.
// impact: XSS before hydration — destructuring does not sanitize.
// improvement: Escape or sanitize after destructuring.
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const { q } = { q: searchQuery };
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
