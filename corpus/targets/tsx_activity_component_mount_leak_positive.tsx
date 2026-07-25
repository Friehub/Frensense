// [frensense]
// observation: `<Activity>` mounts a child component when its visibility becomes true, but the child fetches sensitive data without checking authentication
// impact: unauthorized users can see sensitive data by triggering the Activity visibility (e.g., scrolling, tab focus)
// improvement: wrap the Activity children with an auth guard or pass authentication state before mounting
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

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

export default function TabbedPanel() {
  const [activeTab, setActiveTab] = useState<'overview' | 'admin'>('overview')

  return (
    <div>
      <button onClick={() => setActiveTab('overview')}>Overview</button>
      <button onClick={() => setActiveTab('admin')}>Admin</button>
      <Activity mode="visible" when={activeTab === 'admin'}>
        <AdminDashboard />
      </Activity>
    </div>
  )
}
