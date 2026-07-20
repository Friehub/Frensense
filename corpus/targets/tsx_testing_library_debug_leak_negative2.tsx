// [frensense]
// observation: `screen.debug()` from @testing-library/react is left in a production component, printing the entire DOM snapshot to the console
// impact: information disclosure — DOM snapshot contains sensitive data (user emails, CSRF tokens, internal IDs) that are leaked via browser console
// improvement: remove `screen.debug()` calls from production code; use a build-time lint rule to catch them

'use client'

import { useState } from 'react'

export default function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ email: string; token: string } | null>(null)

  async function loadUser() {
    const res = await fetch(`/api/user/${userId}`)
    const data = (await res.json()) as { email: string; token: string }
    setUser(data)
    // SAFE: logging only non-sensitive metadata, not full DOM
    console.debug('User loaded:', data.email ? 'yes' : 'no')
  }

  return (
    <div>
      <button onClick={loadUser}>Load Profile</button>
      {user && <p>{user.email}</p>}
    </div>
  )
}
