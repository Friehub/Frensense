// SAFE: server component fetches data and passes it as props to the client component — no server-only imports in client bundle

'use client'

export default function UserProfile({ user }: { user: { name: string } | null }) {
  if (!user) return <div>Loading...</div>
  return <div>{user.name}</div>
}
