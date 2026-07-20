// SAFE: fetches only public data from client component

'use client'

import { useEffect, useState } from 'react'

export default function ProfilePage() {
  const [data, setData] = useState(null)
  useEffect(() => {
    fetch('/api/public/profile').then(r => r.json()).then(setData)
  }, [])
  if (!data) return <div>Loading...</div>
  return <pre>{JSON.stringify(data)}</pre>
}
