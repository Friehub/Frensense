// SAFE: uses React.cache to deduplicate identical fetches across parallel routes

import { cache } from 'react'

const getData = cache(async (endpoint: string) => {
  const res = await fetch(endpoint)
  return res.json()
})

export default async function Loading() {
  const [analytics, profile] = await Promise.all([
    getData('/api/analytics'),
    getData('/api/profile'),
  ])
  return (
    <div>
      <span>{analytics.visitors}</span>
      <span>{profile.name}</span>
    </div>
  )
}
