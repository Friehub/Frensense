// [frensense]
// observation: `startTransition` is called but the UI never checks the `isPending` flag, leaving users with no visual feedback during slow transitions
// impact: UI appears stuck or unresponsive — users may click repeatedly, submit forms twice, or navigate away thinking the app is broken
// improvement: wire `isPending` from `useTransition` to a loading indicator or disabled state

'use client'

import { useState, useTransition } from 'react'

export default function SearchPage() {
  const [query, setQuery] = useState('')
  const [isPending, startTransition] = useTransition()
  const [results, setResults] = useState<string[]>([])

  function handleSearch(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value
    setQuery(value)
    startTransition(() => {
      setResults(Array.from({ length: 1000 }, (_, i) => `${value} result ${i}`))
    })
  }

  return (
    <div>
      <input value={query} onChange={handleSearch} placeholder="Search..." disabled={isPending} />
      {/* SAFE: isPending is displayed so user knows work is happening */}
      {isPending && <p>Loading results...</p>}
      <ul>
        {results.map((r, i) => <li key={i}>{r}</li>)}
      </ul>
    </div>
  )
}
