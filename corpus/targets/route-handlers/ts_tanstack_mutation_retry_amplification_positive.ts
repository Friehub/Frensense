// [frensense]
// observation: a mutation has `retry: Infinity` or a very high retry count without a retry delay, causing the client to hammer the server on every failure
// impact: infinite retry storm — a rejected mutation (e.g., 4xx or network error) floods the server with requests, increasing load and potentially causing a DoS
// improvement: set a finite retry count with an exponential backoff delay, or avoid retrying on client errors (4xx)
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { useMutation } from '@tanstack/react-query'

export function useCreateOrder() {
  return useMutation({
    mutationKey: ['order', 'create'],
    mutationFn: async (items: string[]) => {
      const res = await fetch('/api/orders', {
        method: 'POST',
        body: JSON.stringify({ items }),
      })
      return res.json()
    },
    retry: Infinity,
    retryDelay: 0,
  })
}
