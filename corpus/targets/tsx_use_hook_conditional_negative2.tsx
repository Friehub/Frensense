// SAFE: use() is called unconditionally with a noop promise when hidden

'use client'

import { use } from 'react'

function fetchData(id: string): Promise<string> {
  return fetch(`/api/data/${id}`).then(r => r.text())
}

const noopPromise = Promise.resolve(null)

export default function DataWidget({ id, show }: { id: string; show: boolean }) {
  const data = use(show ? fetchData(id) : noopPromise)
  if (!show) return null
  return <div>{data}</div>
}
