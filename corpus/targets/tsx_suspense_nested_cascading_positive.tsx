// [frensense]
// observation: Deeply nested Suspense boundaries (3+ levels) cause cascading sequential loading — each level waits for its parent to resolve before starting its own fetch
// impact: slow page loads as data fetching waterfall increases perceived latency linearly with nesting depth
// improvement: flatten Suspense boundaries or use parallel data fetching with a single Suspense at the top level
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium

'use client'

import { Suspense } from 'react'

export default function Dashboard({ userId }: { userId: string }) {
  return (
    <Suspense fallback={<div>Loading profile...</div>}>
      <ProfileSection userId={userId} />
    </Suspense>
  )
}

async function ProfileSection({ userId }: { userId: string }) {
  const profile = await fetch(`/api/users/${userId}`).then((r) => r.json())
  return (
    <div>
      <h1>{profile.name}</h1>
      <Suspense fallback={<div>Loading orders...</div>}>
        <OrdersSection userId={userId} />
      </Suspense>
    </div>
  )
}

async function OrdersSection({ userId }: { userId: string }) {
  const orders = await fetch(`/api/users/${userId}/orders`).then((r) => r.json())
  return (
    <div>
      <h2>Orders</h2>
      <Suspense fallback={<div>Loading order details...</div>}>
        <OrderDetails orderIds={orders.map((o: { id: string }) => o.id)} />
      </Suspense>
    </div>
  )
}

async function OrderDetails({ orderIds }: { orderIds: string[] }) {
  const details = await fetch(`/api/orders?ids=${orderIds.join(',')}`).then((r) => r.json())
  return <ul>{details.map((d: { id: string; total: number }) => <li key={d.id}>Order #{d.id}: ${d.total}</li>)}</ul>
}
