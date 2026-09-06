// SAFE: component using `use(promise)` is wrapped in a Suspense boundary with a loading fallback

'use client'

import { Suspense, use } from 'react'

async function fetchUser(id: string): Promise<{ name: string }> {
  const res = await fetch(`/api/users/${id}`)
  if (!res.ok) throw new Error('User not found')
  return res.json()
}

function UserProfile({ userId }: { userId: string }) {
  const user = use(fetchUser(userId))
  return <h1>{user.name}</h1>
}

export default function UserPage({ userId }: { userId: string }) {
  return (
    <Suspense fallback={<p>Loading user...</p>}>
      <UserProfile userId={userId} />
    </Suspense>
  )
}
