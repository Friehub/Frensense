// SAFE: parallel route slot uses a shared auth wrapper component

import { requireAdmin } from '@/components/auth-guard'
import { getAnalytics } from '@/lib/analytics'

export default async function AnalyticsSlot() {
  await requireAdmin()
  const data = await getAnalytics()
  return (
    <aside>
      <h2>Analytics</h2>
      <p>Total users: {data.totalUsers}</p>
    </aside>
  )
}
