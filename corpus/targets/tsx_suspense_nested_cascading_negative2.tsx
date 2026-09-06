// SAFE: uses a flat Suspense structure with only 2 levels, and ensures data fetches are not serialized

'use client'

import { Suspense } from 'react'

export default function Dashboard({ userId }: { userId: string }) {
  return (
    <Suspense fallback={<div>Loading dashboard...</div>}>
      <ProfileSection userId={userId} />
      <Suspense fallback={<div>Loading orders...</div>}>
        <OrdersSection userId={userId} />
      </Suspense>
    </Suspense>
  )
}

async function ProfileSection({ userId }: { userId: string }) {
  const profile = await fetch(`/api/users/${userId}`).then((r) => r.json())
  return <h1>{profile.name}</h1>
}

async function OrdersSection({ userId }: { userId: string }) {
  const orders = await fetch(`/api/users/${userId}/orders`).then((r) => r.json())
  return <ul>{orders.map((o: { id: string; total: number }) => <li key={o.id}>Order #{o.id}: ${o.total}</li>)}</ul>
}
