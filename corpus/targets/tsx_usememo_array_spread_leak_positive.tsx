// [frensense]
// observation: `useMemo` returns a new array reference on every render via spread operator, causing downstream `useEffect` to run infinitely
// impact: infinite render loop leads to UI freeze, CPU exhaustion, or API spam
// improvement: memoize the derived value with stable dependencies or use `useRef` for constant arrays

'use client'

import { useMemo, useEffect, useState } from 'react'

export default function SearchResults({ query }: { query: string }) {
  const [results, setResults] = useState<string[]>([])

  const baseItems = useMemo(() => ['apple', 'banana', 'cherry'], [])

  const filtered = useMemo(
    () => [...baseItems].filter((item) => item.includes(query)),
    [baseItems, query]
  )

  useEffect(() => {
    fetch('/api/log', { method: 'POST', body: JSON.stringify({ filtered }) })
  }, [filtered])

  return <ul>{filtered.map((item) => <li key={item}>{item}</li>)}</ul>
}
