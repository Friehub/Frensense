// SAFE: use() is called unconditionally; parent controls visibility via Suspense

'use client'

import { use, Suspense } from 'react'

function fetchData(id: string): Promise<string> {
  return fetch(`/api/data/${id}`).then(r => r.text())
}

function DataInner({ id }: { id: string }) {
  const data = use(fetchData(id))
  return <div>{data}</div>
}

export default function DataWidget({ id, show }: { id: string; show: boolean }) {
  if (!show) return null
  return (
    <Suspense fallback={<div>Loading...</div>}>
      <DataInner id={id} />
    </Suspense>
  )
}
