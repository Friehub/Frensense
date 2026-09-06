// SAFE: conditionally renders the input only after the data has loaded, so the input is always controlled

'use client'

import { useEffect, useState } from 'react'

export default function EditUserForm({ userId }: { userId: string }) {
  const [name, setName] = useState('')
  const [loaded, setLoaded] = useState(false)

  useEffect(() => {
    fetch(`/api/users/${userId}`).then((r) => r.json()).then((data) => {
      setName(data.name ?? '')
      setLoaded(true)
    })
  }, [userId])

  if (!loaded) return <div>Loading form...</div>

  return (
    <form>
      <input value={name} onChange={(e) => setName(e.target.value)} />
    </form>
  )
}
