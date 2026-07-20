// SAFE: useMemo has a proper dependency array so the compiler can cache the result

'use client'

import { useMemo, useState } from 'react'

export default function ProductList({ products }: { products: { price: number }[] }) {
  const [filter, setFilter] = useState('')

  const total = useMemo(() => {
    return products.reduce((sum, p) => sum + p.price, 0)
  }, [products])

  return (
    <div>
      <input value={filter} onChange={(e) => setFilter(e.target.value)} />
      <p>Total: ${total}</p>
    </div>
  )
}
