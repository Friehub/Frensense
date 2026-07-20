// [frensense]
// observation: `onSuccess` callback references a closure variable (e.g., `page` or `searchTerm`) that may be stale by the time the mutation succeeds, because the callback is captured at mutation creation time
// impact: the mutation succeeds with a different value than the user intended — e.g., navigating to a stale page number or showing success for the wrong item
// improvement: use the mutation variables or the returned data from the server in `onSuccess` instead of relying on closure variables

import { useMutation, useQueryClient } from '@tanstack/react-query'

export function useUpdateItem() {
  const queryClient = useQueryClient()

  const searchTerm = getCurrentSearchTerm()

  return useMutation({
    mutationFn: async (itemId: string) => {
      const res = await fetch(`/api/items/${itemId}`, {
        method: 'PATCH',
        body: JSON.stringify({ searchTerm }),
      })
      return res.json()
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['items', { search: searchTerm }] })
    },
  })
}

function getCurrentSearchTerm(): string {
  return (window as any).__searchTerm ?? ''
}
