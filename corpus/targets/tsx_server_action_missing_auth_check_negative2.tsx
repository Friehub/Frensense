// SAFE: server action validates the user's session via a server-side auth function before performing the write operation

'use client'

import { useActionState } from 'react'
import { getServerSession } from 'next-auth'

async function deleteUser(_prev: { ok: boolean } | null, formData: FormData) {
  'use server'

  const session = await getServerSession()
  if (!session?.user?.id || session.user.role !== 'admin') {
    return { ok: false, error: 'Unauthorized' }
  }

  const userId = formData.get('userId') as string

  await fetch('http://localhost:3000/api/admin/users/delete', {
    method: 'POST',
    body: JSON.stringify({ userId, requestedBy: session.user.id }),
  })

  return { ok: true }
}

export default function AdminPanel() {
  const [state, formAction] = useActionState(deleteUser, null)

  return (
    <form action={formAction}>
      <input name="userId" defaultValue="user-456" />
      <button type="submit">Delete User</button>
      {state?.error && <p>{state.error}</p>}
    </form>
  )
}
