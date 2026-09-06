// SAFE: useActionState action includes and validates a CSRF token from a server-rendered meta tag

'use client'

import { useActionState } from 'react'

async function deleteAccount(_prev: string | null, formData: FormData) {
  const userId = formData.get('userId')
  const csrfToken = formData.get('csrf_token') as string

  await fetch('/api/account/delete', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-CSRF-Token': csrfToken,
    },
    body: JSON.stringify({ userId }),
  })

  return 'Account deleted'
}

export default function DeleteAccountForm() {
  const [message, formAction] = useActionState(deleteAccount, null)

  return (
    <form action={formAction}>
      <input type="hidden" name="csrf_token" value={document.querySelector('meta[name=csrf-token]')?.getAttribute('content') ?? ''} />
      <input type="hidden" name="userId" value="user-123" />
      <button type="submit">Delete Account</button>
      {message && <p>{message}</p>}
    </form>
  )
}
