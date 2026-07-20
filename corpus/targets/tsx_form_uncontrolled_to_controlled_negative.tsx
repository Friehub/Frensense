// SAFE: name state is initialized to empty string, so the input is always controlled from the start

'use client'

import { useEffect, useState } from 'react'

export default function EditUserForm({ userId }: { userId: string }) {
  const [name, setName] = useState('')

  useEffect(() => {
    fetch(`/api/users/${userId}`).then((r) => r.json()).then((data) => {
      setName(data.name ?? '')
    })
  }, [userId])

  return (
    <form>
      <input value={name} onChange={(e) => setName(e.target.value)} />
    </form>
  )
}
