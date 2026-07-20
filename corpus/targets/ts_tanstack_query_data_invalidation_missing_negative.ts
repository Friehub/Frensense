// SAFE: the mutation invalidates the related list query on success, ensuring the UI reflects the new data

import { useMutation, useQuery, useQueryClient } from '@tanstack/react-query'

export function useItems() {
  return useQuery({
    queryKey: ['items'],
    queryFn: () => fetch('/api/items').then((r) => r.json()),
  })
}

export function useCreateItem() {
  const queryClient = useQueryClient()

  return useMutation({
    mutationKey: ['items', 'create'],
    mutationFn: async (name: string) => {
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify({ name }),
      })
      return res.json()
    },
    onSuccess: () => {
      queryClient.invalidateQueries({ queryKey: ['items'] })
    },
  })
}
