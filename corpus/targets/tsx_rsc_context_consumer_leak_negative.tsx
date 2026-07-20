// SAFE: receives session data as a prop from the server component instead of consuming server-only context

'use client'

export default function UserGreeting({ session }: { session: { user: { name: string } } | null }) {
  return <h1>Welcome, {session?.user?.name ?? 'Guest'}</h1>
}
