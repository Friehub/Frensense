// [frensense]
// observation: `useDeferredValue` returns the previous stale value while the deferred re-render is pending, causing the UI to display outdated data
// impact: users see stale information (e.g., old search results or expired pricing) during transitions
// improvement: show a pending indicator alongside the deferred value, or use `isPending` from `useTransition`

'use client'

import { useState, useDeferredValue } from 'react'

export default function ProductList() {
  const [search, setSearch] = useState('')
  const deferredSearch = useDeferredValue(search)

  const products = [
    { id: 1, name: 'Laptop', price: 999 },
    { id: 2, name: 'Phone', price: 499 },
  ].filter((p) => p.name.toLowerCase().includes(deferredSearch.toLowerCase()))

  return (
    <div>
      <input onChange={(e) => setSearch(e.target.value)} placeholder="Search products" />
      <p>Showing results for: {deferredSearch}</p>
      <ul>
        {products.map((p) => (
          <li key={p.id}>{p.name} — ${p.price}</li>
        ))}
      </ul>
    </div>
  )
}
