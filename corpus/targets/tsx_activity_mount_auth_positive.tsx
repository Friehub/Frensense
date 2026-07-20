// [frensense]
// observation: `<Activity>` mounts a component on visibility change, and that component fetches data without checking authentication
// impact: an unauthenticated user can trigger the Activity (e.g., by scrolling or tabbing) and access sensitive data
// improvement: wrap Activity children with an authentication guard that checks the session before fetching

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
