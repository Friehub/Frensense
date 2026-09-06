// [frensense]
// observation: `useMemo` is called without a dependency array, so it recomputes on every render — the compiler cannot optimize it
// impact: expensive computations run on every render cycle, causing jank and defeating compiler memoization
// improvement: provide a proper dependency array so the compiler can cache the result between renders

'use client'

import { useMemo, useState } from 'react'

export default function ProductList({ products }: { products: { price: number }[] }) {
  const [filter, setFilter] = useState('')

  const total = useMemo(() => {
    return products.reduce((sum, p) => sum + p.price, 0)
  })

  return (
    <div>
      <input value={filter} onChange={(e) => setFilter(e.target.value)} />
      <p>Total: ${total}</p>
    </div>
  )
}
