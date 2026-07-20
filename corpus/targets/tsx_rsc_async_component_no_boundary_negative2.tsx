// SAFE: wraps the async component in a parent component with error boundary and suspense

import { Suspense } from 'react'
import { ErrorBoundary } from 'react-error-boundary'

async function UserDashboardInner({ userId }: { userId: string }) {
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

function Fallback({ error }: { error: Error }) {
  return <div role="alert">Error: {error.message}</div>
}

export default function UserDashboard({ userId }: { userId: string }) {
  return (
    <ErrorBoundary FallbackComponent={Fallback}>
      <Suspense fallback={<div>Loading user...</div>}>
        <UserDashboardInner userId={userId} />
      </Suspense>
    </ErrorBoundary>
  )
}
