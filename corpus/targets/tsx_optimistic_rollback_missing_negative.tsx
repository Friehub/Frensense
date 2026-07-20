// SAFE: optimistic update is rolled back on server error using a source state update

'use client'

import { useOptimistic, useState, startTransition } from 'react'

type Item = { id: string; name: string }

export default function ItemList() {
  const [items, setItems] = useState<Item[]>([])
  const [optimisticItems, setOptimistic] = useOptimistic(
    items,
    (state, newItem: Item) => [...state, newItem],
  )

  async function addItem(formData: FormData) {
    const name = formData.get('name') as string
    const tempItem: Item = { id: crypto.randomUUID(), name }

    startTransition(() => setOptimistic(tempItem))

    try {
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify({ name }),
      })
      if (!res.ok) throw new Error('Failed')
      const saved = await res.json()
      setItems((prev) => [...prev, saved])
    } catch {
      setItems((prev) => prev.filter((i) => i.id !== tempItem.id))
    }
  }

  return (
    <form action={addItem}>
      <input name="name" required />
      <button type="submit">Add</button>
      <ul>
        {optimisticItems.map((item) => (
          <li key={item.id}>{item.name}</li>
        ))}
      </ul>
    </form>
  )
}
