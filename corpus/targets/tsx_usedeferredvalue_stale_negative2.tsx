// [frensense]
// observation: `useDeferredValue` returns the previous stale value while the deferred re-render is pending, causing the UI to display outdated data
// impact: users see stale information (e.g., old search results or expired pricing) during transitions
// improvement: show a pending indicator alongside the deferred value, or use `isPending` from `useTransition`

'use client'

import { useState, useDeferredValue, useTransition } from 'react'

export default function ProductList() {
  const [search, setSearch] = useState('')
  const [isPending, startTransition] = useTransition()
  const [committedSearch, setCommittedSearch] = useState('')

  function handleSearch(e: React.ChangeEvent<HTMLInputElement>) {
    const value = e.target.value
    setSearch(value)
    startTransition(() => {
      setCommittedSearch(value)
    })
  }

  const products = [
    { id: 1, name: 'Laptop', price: 999 },
    { id: 2, name: 'Phone', price: 499 },
  ].filter((p) => p.name.toLowerCase().includes(committedSearch.toLowerCase()))

  return (
    <div>
      <input onChange={handleSearch} placeholder="Search products" />
      {/* SAFE: isPending shows a loading state instead of stale data */}
      <p>Showing results for: {committedSearch} {isPending && <span className="loading">...</span>}</p>
      <ul>
        {products.map((p) => (
          <li key={p.id}>{p.name} — ${p.price}</li>
        ))}
      </ul>
    </div>
  )
}
