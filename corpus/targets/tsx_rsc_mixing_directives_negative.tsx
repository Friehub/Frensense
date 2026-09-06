// SAFE: uses only 'use client' and delegates server action to imported module

'use client'

import { submitForm } from './actions'
import { useState } from 'react'

export default function ContactForm() {
  const [name, setName] = useState('')
  return (
    <form action={submitForm}>
      <input value={name} onChange={e => setName(e.target.value)} />
      <button type="submit">Send</button>
    </form>
  )
}
