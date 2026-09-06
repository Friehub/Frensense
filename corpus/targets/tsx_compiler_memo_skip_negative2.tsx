// SAFE: items reference is stabilized with useMemo so the compiler can memoize ExpensiveList

'use client'

import { useMemo, useState } from 'react'

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

  const items = useMemo(
    () => [{ id: 1, label: 'Alpha' }, { id: 2, label: 'Beta' }],
    [],
  )

  return (
    <div>
      <input value={query} onChange={(e) => setQuery(e.target.value)} />
      <ExpensiveList items={items} />
    </div>
  )
}
