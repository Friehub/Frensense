// [frensense]
// observation: `<form>` uses a server action reference directly in the `action` prop, which embeds the internal action ID in the client bundle
// impact: the server action's internal identifier (e.g., hashed function name) is exposed in the HTML/JS bundle, enabling CSRF or action enumeration attacks
// improvement: use a wrapper API endpoint or pass the action through a server component with proper CSRF protection
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { createPortal } from 'react'
import { useRef } from 'react'

async function deleteUserAction(formData: FormData) {
  'use server'
  const userId = formData.get('userId')
  await fetch(`/api/admin/users/${userId}`, { method: 'DELETE' })
}

export default function AdminUserRow({ userId, name }: { userId: string; name: string }) {
  const formRef = useRef<HTMLFormElement>(null)

  return (
    <form ref={formRef} action={deleteUserAction}>
      <input type="hidden" name="userId" value={userId} />
      <span>{name}</span>
      <button type="submit">Delete</button>
    </form>
  )
}
