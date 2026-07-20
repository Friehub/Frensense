// SAFE: the server action verifies the user's session and admin role before executing the privileged operation

'use client'

import { useActionState } from 'react'

async function deleteUser(_prev: { ok: boolean } | null, formData: FormData) {
  'use server'

  const userId = formData.get('userId') as string
  const sessionToken = formData.get('session_token') as string

  const authRes = await fetch('http://localhost:3000/api/auth/verify', {
    method: 'POST',
    headers: { Authorization: `Bearer ${sessionToken}` },
  })

  if (!authRes.ok) return { ok: false }
  const session = await authRes.json()
  if (session.role !== 'admin') return { ok: false }

  await fetch('http://localhost:3000/api/admin/users/delete', {
    method: 'POST',
    body: JSON.stringify({ userId }),
  })

  return { ok: true }
}

export default function AdminPanel() {
  const [, formAction] = useActionState(deleteUser, null)

  return (
    <form action={formAction}>
      <input type="hidden" name="session_token" value="..." />
      <input name="userId" defaultValue="user-456" />
      <button type="submit">Delete User</button>
    </form>
  )
}
