// [frensense]
// observation: Client component fetches from an internal API endpoint without sending auth credentials
// impact: unauthenticated API access leaks internal data — the auth token is embedded in client bundle but never sent
// improvement: use server component or pass token from server-side session

'use client'

import { useEffect, useState } from 'react'

export default function ProfilePage() {
  const [data, setData] = useState(null)
  useEffect(() => {
    fetch('/api/internal/profile').then(r => r.json()).then(setData)
  }, [])
  if (!data) return <div>Loading...</div>
  return <pre>{JSON.stringify(data)}</pre>
}
