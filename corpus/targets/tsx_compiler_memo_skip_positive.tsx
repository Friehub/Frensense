// [frensense]
// observation: Function component passes an inline object or array literal as a prop, defeating React Compiler's automatic memoization
// impact: expensive re-renders as child components cannot be memoized — compiler sees unstable prop values and skips optimization
// improvement: stabilize prop references with `useMemo` or `useCallback`, or move the object outside the component

'use client'

import { useState } from 'react'

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
      <ExpensiveList items={[{ id: 1, label: 'Alpha' }, { id: 2, label: 'Beta' }]} />
    </div>
  )
}
