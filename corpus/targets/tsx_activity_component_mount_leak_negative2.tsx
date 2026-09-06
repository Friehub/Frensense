// SAFE: fetches auth state inside Activity before rendering sensitive content, using conditional rendering

'use client'

import { Activity } from 'react'
import { useEffect, useState } from 'react'

export default function TabbedPanel() {
  const [activeTab, setActiveTab] = useState<'overview' | 'admin'>('overview')
  const [session, setSession] = useState<{ role: string } | null>(null)

  useEffect(() => {
    fetch('/api/auth/session').then((r) => r.json()).then(setSession)
  }, [])

  return (
    <div>
      <button onClick={() => setActiveTab('overview')}>Overview</button>
      <button onClick={() => setActiveTab('admin')}>Admin</button>
      <Activity mode="visible" when={activeTab === 'admin'}>
        <AdminDashboard session={session} />
      </Activity>
    </div>
  )
}

function AdminDashboard({ session }: { session: { role: string } | null }) {
  const [sensitiveData, setSensitiveData] = useState<string | null>(null)

  useEffect(() => {
    if (session?.role !== 'admin') return
    fetch('/api/admin/reports').then((r) => r.json()).then(setSensitiveData)
  }, [session])

  if (!session) return <p>Verifying access...</p>
  if (session.role !== 'admin') return <p>Access denied</p>
  return <div>{sensitiveData ? <pre>{JSON.stringify(sensitiveData)}</pre> : 'Loading...'}</div>
}
