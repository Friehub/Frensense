// SAFE: server action is called via a client-side handler that POSTs to a route handler, keeping the action ID server-only

'use client'

import { useRef } from 'react'

export default function AdminUserRow({ userId, name }: { userId: string; name: string }) {
  const formRef = useRef<HTMLFormElement>(null)

  async function handleSubmit(formData: FormData) {
    const id = formData.get('userId')
    await fetch(`/api/admin/users/${id}`, { method: 'DELETE' })
  }

  return (
    <form ref={formRef} action={handleSubmit}>
      <input type="hidden" name="userId" value={userId} />
      <span>{name}</span>
      <button type="submit">Delete</button>
    </form>
  )
}
