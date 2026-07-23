// [frensense]
// observation: a parallel route slot `@analytics` renders without any auth guard, accessible alongside the authenticated main slot
// impact: unauthenticated content renders alongside authenticated pages, leaking data or exposing admin actions
// improvement: add auth check to the parallel route slot's own layout or page

export default async function AnalyticsSlot() {
  const data = await fetch('https://internal-api/admin/analytics').then(r => r.json())
  return (
    <aside>
      <h2>Analytics</h2>
      <p>Total users: {data.totalUsers}</p>
      <p>Revenue: {data.revenue}</p>
    </aside>
  )
}
