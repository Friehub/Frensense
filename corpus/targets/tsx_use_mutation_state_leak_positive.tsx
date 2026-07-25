// [frensense]
// observation: `useMutationState` with a broad filter returns mutations from all components sharing the same query client, leaking other components' mutation data
// impact: sensitive mutation data (e.g., user IDs, PII) from unrelated components is exposed in the UI
// improvement: scope `useMutationState` filters to specific mutation keys that belong to the current component
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { useMutationState } from '@tanstack/react-query'

export default function PaymentStatus() {
  const failedMutations = useMutationState({
    filters: { status: 'error' },
  })

  return (
    <div>
      {failedMutations.map((m) => (
        <p key={m.state.mutationId}>Failed: {JSON.stringify(m.state.variables)}</p>
      ))}
    </div>
  )
}
