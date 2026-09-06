// [frensense]
// observation: User input passes through a helper function before rendering in Suspense fallback.
// impact: XSS before hydration — helper does not sanitize.
// improvement: Sanitize helper output or escape in fallback.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss
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
  const results = await fetch(`/api/search?q=${query}`).then(r => r.json());
  return <ul>{results.map((r: { id: string; title: string }) => <li key={r.id}>{r.title}</li>)}</ul>;
}
