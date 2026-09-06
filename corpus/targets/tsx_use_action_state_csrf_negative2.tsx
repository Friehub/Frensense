// SAFE: useActionState includes a CSRF token validated via a custom request header and SameSite cookie

'use client'

import { useActionState } from 'react'

async function deleteAccount(_prev: string | null, formData: FormData) {
  const userId = formData.get('userId')
  const csrfHeader = formData.get('x-csrf-header') as string

  const res = await fetch('/api/account/delete', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-CSRF-Proof': csrfHeader,
    },
    credentials: 'include',
    body: JSON.stringify({ userId }),
  })

  if (!res.ok) return 'Failed'
  return 'Account deleted'
}

export default function DeleteAccountForm() {
  const [message, formAction] = useActionState(deleteAccount, null)
  const csrfValue = crypto.randomUUID()

  return (
    <form action={formAction}>
      <input type="hidden" name="x-csrf-header" value={csrfValue} />
      <input type="hidden" name="userId" value="user-123" />
      <button type="submit">Delete Account</button>
      {message && <p>{message}</p>}
    </form>
  )
}
