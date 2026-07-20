// [frensense]
// observation: the `use()` hook is called inside a conditional block, violating the rules of hooks
// impact: React may crash or produce inconsistent renders because hooks must not be called conditionally — the promise resource may not be registered
// improvement: move `use()` outside the conditional or use a Suspense boundary at the parent level

'use client'

import { use, Suspense } from 'react'

function fetchData(id: string): Promise<string> {
  return fetch(`/api/data/${id}`).then(r => r.text())
}

export default function DataWidget({ id, show }: { id: string; show: boolean }) {
  if (!show) return null
  const data = use(fetchData(id))
  return <div>{data}</div>
}
