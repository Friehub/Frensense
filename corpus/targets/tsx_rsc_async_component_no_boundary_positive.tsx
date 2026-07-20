// [frensense]
// observation: Async server component throws an error (e.g., fetch failure, null access) without being wrapped in an error boundary
// impact: the entire page crashes with a 500 error — no fallback UI is shown, leaving the user with a blank error screen
// improvement: wrap async server components with `<Suspense>` and `<ErrorBoundary>`, or use error.tsx in Next.js

export default async function UserDashboard({ userId }: { userId: string }) {
  const res = await fetch(`https://api.example.com/users/${userId}`)
  if (!res.ok) throw new Error('Failed to fetch user')
  const user = await res.json()

  return (
    <div>
      <h1>{user.name}</h1>
      <p>{user.email}</p>
    </div>
  )
}
