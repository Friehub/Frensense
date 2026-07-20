// SAFE: React escaping protects helper output
'use client'
import { Suspense } from 'react'
function getQueryParam(q: string): string { return q; }
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = getQueryParam(searchQuery);
  return (
    <div>
      <Suspense fallback={<div>Searching for: {q}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const encoded = encodeURIComponent(query);
  const results = await fetch(`/api/search?q=${encoded}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
