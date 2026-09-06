// SAFE: items array is defined outside the component — stable reference allows compiler memoization

'use client'

import { useState } from 'react'

const DEFAULT_ITEMS = [
  { id: 1, label: 'Alpha' },
  { id: 2, label: 'Beta' },
]

function ExpensiveList({ items }: { items: { id: number; label: string }[] }) {
  return (
    <ul>
      {items.map((item) => (
        <li key={item.id}>{item.label}</li>
      ))}
    </ul>
  )
}

export default function SearchPage() {
  const [query, setQuery] = useState('')

  return (
    <div>
      <input value={query} onChange={(e) => setQuery(e.target.value)} />
      <ExpensiveList items={DEFAULT_ITEMS} />
    </div>
  )
}
