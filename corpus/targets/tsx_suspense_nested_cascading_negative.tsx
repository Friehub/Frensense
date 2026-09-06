// SAFE: data fetches are parallelized at the top level and a single Suspense boundary wraps the entire section

'use client'

import { Suspense } from 'react'

export default function Dashboard({ userId }: { userId: string }) {
  return (
    <Suspense fallback={<div>Loading dashboard...</div>}>
      <DashboardContent userId={userId} />
    </Suspense>
  )
}

async function DashboardContent({ userId }: { userId: string }) {
  const [profile, orders] = await Promise.all([
    fetch(`/api/users/${userId}`).then((r) => r.json()),
    fetch(`/api/users/${userId}/orders`).then((r) => r.json()),
  ])

  const details = await fetch(`/api/orders?ids=${orders.map((o: { id: string }) => o.id).join(',')}`).then((r) => r.json())

  return (
    <div>
      <h1>{profile.name}</h1>
      <h2>Orders ({orders.length})</h2>
      <ul>{details.map((d: { id: string; total: number }) => <li key={d.id}>Order #{d.id}: ${d.total}</li>)}</ul>
    </div>
  )
}
