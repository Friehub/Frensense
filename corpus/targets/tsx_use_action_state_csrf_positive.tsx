// [frensense]
// observation: `useActionState` action performs a state-changing fetch without including or validating a CSRF token
// impact: cross-site request forgery — an attacker can submit the form from a malicious site and the server accepts the state change
// improvement: include a server-validated CSRF token in the action payload or use SameSite cookies with a custom header check

'use client'

import { useActionState } from 'react'

async function deleteAccount(_prev: string | null, formData: FormData) {
  const userId = formData.get('userId')

  await fetch('/api/account/delete', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ userId }),
  })

  return 'Account deleted'
}

export default function DeleteAccountForm() {
  const [message, formAction] = useActionState(deleteAccount, null)

  return (
    <form action={formAction}>
      <input type="hidden" name="userId" value="user-123" />
      <button type="submit">Delete Account</button>
      {message && <p>{message}</p>}
    </form>
  )
}
