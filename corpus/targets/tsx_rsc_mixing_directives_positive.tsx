// [frensense]
// observation: component uses both 'use client' and 'use server' directives — this is ambiguous and one will silently override the other
// impact: directive precedence causes unexpected server-only or client-only behaviour, potentially leaking server context to client bundle
// improvement: pick one directive; use a separate server action file for server-only logic

'use client'
'use server'

import { useState } from 'react'

export function submitForm(data: FormData) {
  const name = data.get('name')
  return fetch('/api/submit', { method: 'POST', body: data })
}

export default function ContactForm() {
  const [name, setName] = useState('')
  return (
    <form action={submitForm}>
      <input value={name} onChange={e => setName(e.target.value)} />
      <button type="submit">Send</button>
    </form>
  )
}
