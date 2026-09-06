// SAFE: includes a CSRF token from a server-rendered meta tag in the form submission

'use client'

import { useActionState } from 'react'

async function submitOrder(_prev: string | null, formData: FormData) {
  const csrfToken = formData.get('csrf_token')
  const itemId = formData.get('itemId')
  const quantity = formData.get('quantity')

  const res = await fetch('/api/orders', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', 'X-CSRF-Token': csrfToken as string },
    body: JSON.stringify({ itemId, quantity }),
  })

  if (!res.ok) return 'Order failed'
  return 'Order placed'
}

export default function OrderForm() {
  const [message, formAction] = useActionState(submitOrder, null)

  return (
    <form action={formAction}>
      <input type="hidden" name="csrf_token" value={document.querySelector('meta[name=csrf-token]')?.getAttribute('content') ?? ''} />
      <input name="itemId" defaultValue="prod-1" />
      <input name="quantity" type="number" defaultValue={1} />
      <button type="submit">Place Order</button>
      {message && <p>{message}</p>}
    </form>
  )
}
