// SAFE: fallback uses a generic message without user-controlled content
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  return (
    <div>
      <Suspense fallback={<div>Loading search results...</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
