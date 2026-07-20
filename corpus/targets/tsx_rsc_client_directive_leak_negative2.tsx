// SAFE: client component fetches from a public API endpoint instead of directly using server-only modules

'use client'

import { useEffect, useState } from 'react'

export default function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ name: string } | null>(null)

  useEffect(() => {
    fetch(`/api/users/${userId}`).then((r) => r.json()).then(setUser)
  }, [userId])

  if (!user) return <div>Loading...</div>
  return <div>{user.name}</div>
}
