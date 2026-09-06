// [frensense]
// observation: `useTransition` pending state is used to display loading text that includes internal state descriptions or resource identifiers
// impact: users can infer internal loading states, resource IDs, or operation types from the pending indicator
// improvement: use generic loading messages that do not reveal internal operation details

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
        {isPending ? 'Updating role for user 123...' : 'Update'}
      </button>
    </div>
  )
}
