// SAFE: useMutation is defined inside an effect or event handler so the closure captures fresh values

import { useMutation, useQueryClient } from '@tanstack/react-query'

export function useUpdateItem(searchTerm: string) {
  const queryClient = useQueryClient()

  const mutation = useMutation({
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

  return mutation
}
