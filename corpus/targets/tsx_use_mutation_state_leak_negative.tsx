// SAFE: useMutationState is scoped to a specific mutation key, preventing cross-component data leaks

'use client'

import { useMutationState } from '@tanstack/react-query'

const PAYMENT_MUTATION_KEY = ['payment', 'create']

export default function PaymentStatus() {
  const failedMutations = useMutationState({
    filters: { status: 'error', mutationKey: PAYMENT_MUTATION_KEY },
  })

  return (
    <div>
      {failedMutations.map((m) => (
        <p key={m.state.mutationId}>Failed payment: {m.state.variables}</p>
      ))}
    </div>
  )
}
