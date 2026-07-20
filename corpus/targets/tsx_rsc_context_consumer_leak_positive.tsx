// [frensense]
// observation: Client component uses `useContext` to consume a context that was created and provided only on the server side
// impact: the context value is `undefined` on the client, causing runtime errors or rendering empty/incorrect UI without warning
// improvement: ensure context is also provided on the client, or pass data as props from server to client

'use client'

import { useContext } from 'react'
import { ServerSessionContext } from '@/lib/server-context'

export default function UserGreeting() {
  const session = useContext(ServerSessionContext)

  return <h1>Welcome, {session?.user?.name ?? 'Guest'}</h1>
}
