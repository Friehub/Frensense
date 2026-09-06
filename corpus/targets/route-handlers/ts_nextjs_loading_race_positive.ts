// [frensense]
// observation: parallel route loading.tsx fires multiple data fetches that race without coordination
// impact: stale or inconsistent data rendered; one fetch overwrites the result of another
// improvement: use a shared cache key or deduplicate fetches
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

import { use } from 'react'

async function getAnalytics() {
  const res = await fetch('/api/analytics/slow')
  return res.json()
}

async function getProfile() {
  const res = await fetch('/api/profile/fast')
  return res.json()
}

export default function Loading() {
  const analytics = use(getAnalytics())
  const profile = use(getProfile())
  return (
    <div>
      <span>{analytics.visitors}</span>
      <span>{profile.name}</span>
    </div>
  )
}
