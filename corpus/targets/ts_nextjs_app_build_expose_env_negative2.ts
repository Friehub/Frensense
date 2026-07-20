// SAFE: Private env vars are fetched server-side via a server action, never compiled into the client bundle

'use client'

import { useState, useEffect } from 'react'

export default function DashboardPage() {
  const [config, setConfig] = useState<{ apiUrl: string } | null>(null)

  useEffect(() => {
    fetch('/api/config')
      .then((r) => r.json())
      .then(setConfig)
  }, [])

  if (!config) return <div>Loading...</div>

  return (
    <div>
      <h1>Dashboard</h1>
    </div>
  )
}
