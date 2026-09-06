// SAFE: submit button is disabled when form submission is pending, preventing duplicate submissions

'use client'

import { useFormStatus } from 'react-dom'

function SubmitButton() {
  const { pending } = useFormStatus()

  return <button type="submit" disabled={pending}>{pending ? 'Submitting...' : 'Submit'}</button>
}

export default function OrderForm() {
  return (
    <form action="/api/orders" method="POST">
      <input name="productId" defaultValue="prod-1" />
      <SubmitButton />
    </form>
  )
}
