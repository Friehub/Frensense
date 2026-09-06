// SAFE: intermediate variable is rendered via React's default escaping (safe)
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const q = searchQuery;
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
