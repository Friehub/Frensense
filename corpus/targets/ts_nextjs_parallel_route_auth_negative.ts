// SAFE: parallel route slot checks auth and returns null if unauthenticated

import { redirect } from 'next/navigation'
import { getSession } from '@/lib/session'

export default async function AnalyticsSlot() {
  const session = await getSession()
  if (!session) return null
  const data = await fetch('https://internal-api/admin/analytics').then(r => r.json())
  return (
    <aside>
      <h2>Analytics</h2>
      <p>Total users: {data.totalUsers}</p>
    </aside>
  )
}
