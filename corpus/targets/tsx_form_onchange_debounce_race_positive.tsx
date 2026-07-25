// [frensense]
// observation: debounced onChange handler uses a stale closure over the input value — the debounce fires with old data after rapid input changes
// impact: the server receives stale or inconsistent data; in search or autocomplete, the wrong term may be sent, causing data leakage across users
// improvement: use a ref to track the latest value or cancel the debounce on each keystroke
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

'use client'

import { useState } from 'react'

function debounce<T extends (...args: unknown[]) => void>(fn: T, ms: number): T {
  let timer: ReturnType<typeof setTimeout>
  return ((...args: unknown[]) => {
    clearTimeout(timer)
    timer = setTimeout(() => fn(...args), ms)
  }) as T
}

export default function SearchPage() {
  const [query, setQuery] = useState('')
  const search = debounce((q: string) => {
    fetch(`/api/search?q=${q}`)
  }, 300)
  return (
    <input
      value={query}
      onChange={e => {
        setQuery(e.target.value)
        search(e.target.value)
      }}
    />
  )
}
