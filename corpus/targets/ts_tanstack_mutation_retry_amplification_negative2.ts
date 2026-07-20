// SAFE: mutation retry is limited and uses exponential backoff; 4xx errors are not retried

import { useMutation } from '@tanstack/react-query'

export function useCreateOrder() {
  return useMutation({
    mutationKey: ['order', 'create'],
    mutationFn: async (items: string[]) => {
      const res = await fetch('/api/orders', {
        method: 'POST',
        body: JSON.stringify({ items }),
      })
      if (res.status >= 400 && res.status < 500) {
        throw new Error('Client error — not retrying')
      }
      return res.json()
    },
    retry: (failureCount, error) => {
      if (error instanceof Error && error.message.includes('Client error')) return false
      return failureCount < 3
    },
    retryDelay: (attempt) => Math.min(1000 * 2 ** attempt, 15000),
  })
}
