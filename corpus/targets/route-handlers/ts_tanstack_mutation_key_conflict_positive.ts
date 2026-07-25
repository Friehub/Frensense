// [frensense]
// observation: two unrelated mutations share the same mutation key, so `useMutationState` and cache observers from one mutation fire callbacks for the other
// impact: cross-component side effects — a payment success listener fires when a unrelated profile update mutation completes
// improvement: use unique, descriptive mutation keys that include a domain prefix
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { useMutation, useMutationState } from '@tanstack/react-query'

export function useUpdateProfile() {
  return useMutation({
    mutationKey: ['update'],
    mutationFn: async (data: { name: string }) => {
      const res = await fetch('/api/profile', {
        method: 'PUT',
        body: JSON.stringify(data),
      })
      return res.json()
    },
  })
}

export function useUpdatePassword() {
  return useMutation({
    mutationKey: ['update'],
    mutationFn: async (data: { password: string }) => {
      const res = await fetch('/api/password', {
        method: 'PUT',
        body: JSON.stringify(data),
      })
      return res.json()
    },
  })
}

export function PaymentSuccessListener() {
  const mutations = useMutationState({
    filters: { mutationKey: ['update'], status: 'success' },
  })

  const latest = mutations[mutations.length - 1]

  // This fires for profile updates too — not just payments
  return <p>Payment succeeded: {JSON.stringify(latest?.state.variables)}</p>
}
