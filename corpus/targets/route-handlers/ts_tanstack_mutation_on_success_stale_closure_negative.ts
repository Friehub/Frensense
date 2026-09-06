// SAFE: onSuccess uses the mutation variables and server response instead of stale closure values

import { useMutation, useQueryClient } from '@tanstack/react-query'

export function useUpdateItem() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationFn: async ({ itemId, searchTerm }: { itemId: string; searchTerm: string }) => {
      const res = await fetch(`/api/items/${itemId}`, {
        method: 'PATCH',
        body: JSON.stringify({ searchTerm }),
      })
      return res.json()
    },
    onSuccess: (_data, variables) => {
      queryClient.invalidateQueries({ queryKey: ['items', { search: variables.searchTerm }] })
    },
  })
}
