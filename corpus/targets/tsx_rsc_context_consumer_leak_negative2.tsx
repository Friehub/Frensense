// SAFE: creates a client-side context as well, with a fallback default value

'use client'

import { createContext, useContext } from 'react'

type Session = { user: { name: string } } | null

const ClientSessionContext = createContext<Session>(null)

export function ClientSessionProvider({ children, session }: { children: React.ReactNode; session: Session }) {
  return <ClientSessionContext.Provider value={session}>{children}</ClientSessionContext.Provider>
}

export default function UserGreeting() {
  const session = useContext(ClientSessionContext)

  return <h1>Welcome, {session?.user?.name ?? 'Guest'}</h1>
}
