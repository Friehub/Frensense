// [frensense]
// observation: a server action performs a privileged operation (e.g., delete user, change role) without verifying the user's authentication or authorization server-side
// impact: privilege escalation — an unauthenticated or low-privilege user can invoke the server action to perform admin-level operations
// improvement: add authentication and authorization checks at the top of every server action before executing the operation
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

'use client'

import { useActionState } from 'react'

async function deleteUser(_prev: { ok: boolean } | null, formData: FormData) {
  'use server'

  const userId = formData.get('userId') as string

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
      <input name="userId" defaultValue="user-456" />
      <button type="submit">Delete User</button>
    </form>
  )
}
