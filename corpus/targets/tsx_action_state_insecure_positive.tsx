// [frensense]
// observation: `useActionState` returns the result of a server action that includes sensitive fields like `internalNote` or `token` alongside the expected form state
// impact: sensitive internal data is rendered back to the client in the action state response, visible in the DOM or devtools
// improvement: strip sensitive fields from the server action return value before returning to useActionState

'use client'

import { useActionState } from 'react'
import { submitOrder } from './actions'

export default function OrderForm() {
  const [state, formAction] = useActionState(submitOrder, null)
  return (
    <form action={formAction}>
      <input name="productId" />
      <button type="submit">Order</button>
      {state && <p>Order ID: {state.orderId} — Internal note: {state.internalNote}</p>}
    </form>
  )
}
