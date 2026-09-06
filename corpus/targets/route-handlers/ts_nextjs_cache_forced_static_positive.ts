// [frensense]
// observation: `force-static` segment config applied to a page with dynamic user-specific data
// impact: stale content served to all users; user A sees user B's data
// improvement: use `dynamic = 'force-dynamic'` or revalidate properly

import { use } from 'react'

export const dynamic = 'force-static'

async function getDashboard() {
  const res = await fetch('https://api.internal/dashboard', { cache: 'force-cache' })
  return res.json()
}

export default function DashboardPage() {
  const data = use(getDashboard())
  return <div>Welcome back, {data.user.name}</div>
}
