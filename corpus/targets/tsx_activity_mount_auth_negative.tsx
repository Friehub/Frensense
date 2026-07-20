// SAFE: Activity children are wrapped with an authentication guard that checks session before fetching

'use client'

import { Activity } from 'react'
import { useEffect, useState } from 'react'

function FinancialReport() {
  const [data, setData] = useState<unknown>(null)

  useEffect(() => {
    fetch('/api/financials/internal').then((r) => r.json()).then(setData)
  }, [])

  return <div>{data ? <pre>{JSON.stringify(data)}</pre> : 'Loading report...'}</div>
}

function AuthGuard({ children }: { children: React.ReactNode }) {
  const [session, setSession] = useState<{ role: string } | null>(null)

  useEffect(() => {
    fetch('/api/auth/session').then((r) => r.json()).then(setSession)
  }, [])

  if (!session) return <p>Checking access...</p>
  if (session.role !== 'admin') return <p>Access denied</p>
  return <>{children}</>
}

export default function Dashboard() {
  const [showReport, setShowReport] = useState(false)

  return (
    <div>
      <button onClick={() => setShowReport((v) => !v)}>Toggle Report</button>
      <Activity mode="visible" when={showReport}>
        <AuthGuard>
          <FinancialReport />
        </AuthGuard>
      </Activity>
    </div>
  )
}
