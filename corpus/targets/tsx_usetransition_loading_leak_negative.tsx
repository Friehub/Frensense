// SAFE: uses a generic loading message without internal operation details

'use client'

import { useState, useTransition } from 'react'

export default function AdminPanel() {
  const [isPending, startTransition] = useTransition()
  const [result, setResult] = useState<string | null>(null)

  function handleDeleteUser(userId: string) {
    startTransition(async () => {
      const res = await fetch(`/api/admin/users/${userId}/delete`, { method: 'DELETE' })
      const data = await res.json()
      setResult(data.message)
    })
  }

  return (
    <div>
      <button onClick={() => handleDeleteUser('u-42')} disabled={isPending}>
        {isPending ? 'Processing...' : 'Delete User'}
      </button>
      {result && <p>{result}</p>}
    </div>
  )
}
