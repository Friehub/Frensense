// [frensense]
// observation: a server action receives FormData and updates server state but never invalidates the TanStack Query cache, so the UI still shows stale data
// impact: users see outdated data after a server mutation because the query cache is not invalidated or updated
// improvement: call `queryClient.invalidateQueries` in the server action or the calling component after a successful mutation

'use client'

import { useQuery } from '@tanstack/react-query'
import { useActionState } from 'react'

async function addItem(_prev: { ok: boolean } | null, formData: FormData) {
  'use server'

  const name = formData.get('name') as string
  await fetch('http://localhost:3000/api/items', {
    method: 'POST',
    body: JSON.stringify({ name }),
  })

  return { ok: true }
}

export default function ItemsPage() {
  const { data: items } = useQuery<string[]>({
    queryKey: ['items'],
    queryFn: () => fetch('/api/items').then((r) => r.json()),
  })

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
