// SAFE: uses a spinner without any text

'use client'

import { useState, useTransition } from 'react'

export default function UserUpdateForm() {
  const [name, setName] = useState('')
  const [isPending, startTransition] = useTransition()

  function handleSubmit() {
    startTransition(async () => {
      await fetch('/api/admin/users/123/update-role', { method: 'POST' })
    })
  }

  return (
    <div>
      <button onClick={handleSubmit} disabled={isPending}>
        {isPending ? <span className="spinner" /> : 'Update'}
      </button>
    </div>
  )
}
