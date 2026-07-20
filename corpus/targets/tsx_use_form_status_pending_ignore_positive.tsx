// [frensense]
// observation: `useFormStatus` is used but the `pending` state is not checked before disabling the submit button, allowing multiple submissions while a previous action is still in flight
// impact: duplicate form submissions cause duplicate server mutations (e.g., double charge, duplicate order) since the action runs multiple times
// improvement: disable the submit button when `pending` is true from `useFormStatus`

'use client'

import { useFormStatus } from 'react-dom'

function SubmitButton() {
  const { pending } = useFormStatus()

  return <button type="submit">Submit</button>
}

export default function OrderForm() {
  return (
    <form action="/api/orders" method="POST">
      <input name="productId" defaultValue="prod-1" />
      <SubmitButton />
    </form>
  )
}
