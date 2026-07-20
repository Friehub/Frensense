// SAFE: the query client invalidates the items query after the server action completes

'use client'

import { useQuery, useQueryClient } from '@tanstack/react-query'
import { useActionState } from 'react'

export default function ItemsPage() {
  const queryClient = useQueryClient()

  const { data: items } = useQuery<string[]>({
    queryKey: ['items'],
    queryFn: () => fetch('/api/items').then((r) => r.json()),
  })

  async function addItem(_prev: { ok: boolean } | null, formData: FormData) {
    'use server'

    const name = formData.get('name') as string
    await fetch('http://localhost:3000/api/items', {
      method: 'POST',
      body: JSON.stringify({ name }),
    })

    queryClient.invalidateQueries({ queryKey: ['items'] })
    return { ok: true }
  }

  const [, formAction] = useActionState(addItem, null)

  return (
    <form action={formAction}>
      <input name="name" required />
      <button type="submit">Add</button>
      <ul>
        {items?.map((item) => <li key={item}>{item}</li>)}
      </ul>
    </form>
  )
}
