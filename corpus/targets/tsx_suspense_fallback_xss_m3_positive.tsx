// [frensense]
// observation: User input flows through two assignments before rendering in Suspense fallback.
// impact: XSS before hydration via multi-hop user content.
// improvement: Sanitize or escape before fallback rendering.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
'use client'
import { Suspense } from 'react'
export default function SearchPage({ searchQuery }: { searchQuery: string }) {
  const a = searchQuery;
  const b = a;
  return (
    <div>
      <Suspense fallback={<div>Searching for: {b}</div>}>
        <SearchResults query={searchQuery} />
      </Suspense>
    </div>
  )
}
async function SearchResults({ query }: { query: string }) {
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
