// SAFE: form submission is gated by useFormStatus pending flag which prevents double-submission

'use client'

import { useFormStatus } from 'react-dom'

function SubmitButton() {
  const { pending, action } = useFormStatus()

  return (
    <button type="submit" disabled={pending} data-action={action}>
      {pending ? 'Saving...' : 'Save'}
    </button>
  )
}

export default function SettingsForm() {
  return (
    <form action="/api/settings" method="POST">
      <input name="theme" defaultValue="dark" />
      <SubmitButton />
    </form>
  )
}
