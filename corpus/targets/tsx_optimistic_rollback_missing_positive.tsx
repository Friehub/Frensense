// [frensense]
// observation: `useOptimistic` applies an optimistic UI value but the component never rolls back when the underlying server mutation fails
// impact: users see permanently stale data after a failed server mutation, leading to confusion and data inconsistency
// improvement: wrap the mutation in a try/catch and update the source state to remove the optimistic entry on error

'use client'

import { useOptimistic, useState } from 'react'

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

    setOptimistic(tempItem)

    const res = await fetch('/api/items', {
      method: 'POST',
      body: JSON.stringify({ name }),
    })

    if (!res.ok) {
      console.error('Failed to add item')
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
