// SAFE: uses `dynamic = 'force-dynamic'` to always render fresh data

import { use } from 'react'

export const dynamic = 'force-dynamic'

async function getDashboard(sessionId: string) {
  const res = await fetch('https://api.internal/dashboard', {
    headers: { Authorization: `Bearer ${sessionId}` },
  })
  return res.json()
}

export default function DashboardPage() {
  const data = use(getDashboard('session-token'))
  return <div>Welcome back, {data.user.name}</div>
}
