// SAFE: uses ISR with a short revalidation period instead of force-static

import { use } from 'react'

export const revalidate = 30

async function getDashboard() {
  const res = await fetch('https://api.internal/dashboard', { next: { revalidate: 30 } })
  return res.json()
}

export default function DashboardPage() {
  const data = use(getDashboard())
  return <div>Welcome back, {data.user.name}</div>
}
