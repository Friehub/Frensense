// SAFE: Activity contains a component that checks authentication before fetching sensitive data

'use client'

import { Activity } from 'react'
import { useEffect, useState } from 'react'

function FinancialReport() {
  const [session, setSession] = useState<{ role: string } | null>(null)
  const [data, setData] = useState<unknown>(null)

  useEffect(() => {
    fetch('/api/auth/session')
      .then((r) => r.json())
      .then((s) => {
        setSession(s)
        if (s.role === 'admin') {
          return fetch('/api/financials/internal').then((r) => r.json())
        }
        return null
      })
      .then(setData)
  }, [])

  if (!session) return <p>Loading...</p>
  if (session.role !== 'admin') return <p>Access denied</p>
  return <div>{data ? <pre>{JSON.stringify(data)}</pre> : 'Loading report...'}</div>
}

export default function Dashboard() {
  const [showReport, setShowReport] = useState(false)

  return (
    <div>
      <button onClick={() => setShowReport((v) => !v)}>Toggle Report</button>
      <Activity mode="visible" when={showReport}>
        <FinancialReport />
      </Activity>
    </div>
  )
}
