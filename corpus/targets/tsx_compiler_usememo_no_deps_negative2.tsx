// SAFE: refactors the computation to a simple variable since there are no dependencies — no useMemo needed

'use client'

import { useState } from 'react'

export default function ProductList({ products }: { products: { price: number }[] }) {
  const [filter, setFilter] = useState('')

  const total = products.reduce((sum, p) => sum + p.price, 0)

  return (
    <div>
      <input value={filter} onChange={(e) => setFilter(e.target.value)} />
      <p>Total: ${total}</p>
    </div>
  )
}
