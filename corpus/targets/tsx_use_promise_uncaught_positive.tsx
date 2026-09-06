// [frensense]
// observation: `use(promise)` reads a promise directly without being wrapped in a `<Suspense>` boundary, so if the promise rejects the error propagates uncaught
// impact: the entire component tree crashes with an unhandled promise rejection instead of showing a fallback UI
// improvement: wrap the component using `use(promise)` in a `<Suspense>` boundary with a fallback, and optionally an error boundary

'use client'

import { use } from 'react'

async function fetchUser(id: string): Promise<{ name: string }> {
  const res = await fetch(`/api/users/${id}`)
  if (!res.ok) throw new Error('User not found')
  return res.json()
}

export default function UserProfile({ userId }: { userId: string }) {
  const user = use(fetchUser(userId))

  return <h1>{user.name}</h1>
}
