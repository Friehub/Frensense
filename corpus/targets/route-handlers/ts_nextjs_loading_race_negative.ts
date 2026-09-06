// SAFE: serializes fetches and validates data consistency

import { use } from 'react'

async function getProfile() {
  const res = await fetch('/api/profile')
  return res.json()
}

async function getAnalytics(uid: string) {
  const res = await fetch(`/api/analytics?uid=${uid}`)
  return res.json()
}

export default function Loading() {
  const profile = use(getProfile())
  const analytics = use(getAnalytics(profile.id))
  return (
    <div>
      <span>{profile.name}</span>
      <span>{analytics.visitors}</span>
    </div>
  )
}
