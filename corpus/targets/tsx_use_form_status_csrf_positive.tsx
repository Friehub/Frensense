// [frensense]
// observation: `useActionState` form handler does not include or validate a CSRF token before processing the request
// impact: attackers can forge cross-site requests that execute state-changing actions without user consent
// improvement: include a server-validated CSRF token in the form action or use SameSite cookies with CSRF header checks
// cwe: CWE-352
// cvss: 8.8
// owasp: A01:2021
// severity: High

'use client'

import { useActionState } from 'react'

async function submitOrder(_prev: string | null, formData: FormData) {
  const itemId = formData.get('itemId')
  const quantity = formData.get('quantity')

  const res = await fetch('/api/orders', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
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
