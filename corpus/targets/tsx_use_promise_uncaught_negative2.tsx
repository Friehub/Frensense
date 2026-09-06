// SAFE: use(promise) is wrapped in both Suspense and an ErrorBoundary for graceful error handling

'use client'

import { Suspense, use, Component, type ReactNode } from 'react'

interface ErrorBoundaryProps { children: ReactNode; fallback: ReactNode }
interface ErrorBoundaryState { hasError: boolean }

class SimpleErrorBoundary extends Component<ErrorBoundaryProps, ErrorBoundaryState> {
  state: ErrorBoundaryState = { hasError: false }

  static getDerivedStateFromError(): ErrorBoundaryState {
    return { hasError: true }
  }

  render() {
    if (this.state.hasError) return this.props.fallback
    return this.props.children
  }
}

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
    <SimpleErrorBoundary fallback={<p>Could not load user</p>}>
      <Suspense fallback={<p>Loading user...</p>}>
        <UserProfile userId={userId} />
      </Suspense>
    </SimpleErrorBoundary>
  )
}
