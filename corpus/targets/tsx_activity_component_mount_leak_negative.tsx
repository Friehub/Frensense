// SAFE: Activity children are guarded by an authentication check before mounting

'use client'

import { Activity } from 'react'
import { useEffect, useState } from 'react'

function AdminDashboard() {
  const [sensitiveData, setSensitiveData] = useState<string | null>(null)

  useEffect(() => {
    fetch('/api/admin/reports').then((r) => r.json()).then(setSensitiveData)
  }, [])

  return <div>{sensitiveData ? <pre>{JSON.stringify(sensitiveData)}</pre> : 'Loading...'}</div>
}

function AuthGuard({ children }: { children: React.ReactNode }) {
  const [session, setSession] = useState<{ role: string } | null>(null)

  useEffect(() => {
    fetch('/api/auth/session').then((r) => r.json()).then(setSession)
  }, [])

  if (!session) return <p>Verifying access...</p>
  if (session.role !== 'admin') return <p>Access denied</p>
  return <>{children}</>
}

export default function TabbedPanel() {
  const [activeTab, setActiveTab] = useState<'overview' | 'admin'>('overview')

  return (
    <div>
      <button onClick={() => setActiveTab('overview')}>Overview</button>
      <button onClick={() => setActiveTab('admin')}>Admin</button>
      <Activity mode="visible" when={activeTab === 'admin'}>
        <AuthGuard>
          <AdminDashboard />
        </AuthGuard>
      </Activity>
    </div>
  )
}
