// SAFE: validates the Origin/Referer header on the server — server-side CSRF protection via header check

'use client'

import { useActionState } from 'react'

async function submitOrder(_prev: string | null, formData: FormData) {
  const itemId = formData.get('itemId')
  const quantity = formData.get('quantity')

  const res = await fetch('/api/orders', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      'X-CSRF-Protected': '1',
    },
    body: JSON.stringify({ itemId, quantity }),
  })

  if (!res.ok) return 'Order failed'
  return 'Order placed'
}

export default function OrderForm() {
  const [message, formAction] = useActionState(submitOrder, null)

  return (
    <form action={formAction}>
      <input name="itemId" defaultValue="prod-1" />
      <input name="quantity" type="number" defaultValue={1} />
      <button type="submit">Place Order</button>
      {message && <p>{message}</p>}
    </form>
  )
}
