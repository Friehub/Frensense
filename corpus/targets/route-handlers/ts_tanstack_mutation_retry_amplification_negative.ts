// SAFE: mutation retry is limited to 3 attempts with exponential backoff, preventing a request storm

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
    retry: 3,
    retryDelay: (attempt) => Math.min(1000 * 2 ** attempt, 30000),
  })
}
