// SAFE: wraps async fetch in a try/catch and returns a safe fallback instead of throwing

export default async function UserDashboard({ userId }: { userId: string }) {
  try {
    const res = await fetch(`https://api.example.com/users/${userId}`)
    if (!res.ok) throw new Error('Fetch failed')
    const user = await res.json()

    return (
      <div>
        <h1>{user.name}</h1>
        <p>{user.email}</p>
      </div>
    )
  } catch {
    return (
      <div role="alert">
        <p>Could not load user data. Please try again later.</p>
      </div>
    )
  }
}
