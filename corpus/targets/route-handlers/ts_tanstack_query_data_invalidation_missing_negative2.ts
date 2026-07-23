// SAFE: the mutation updates the query cache directly and then invalidates related queries

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
      if (!res.ok) throw new Error('Failed to create')
      return res.json()
    },
    onSuccess: (newItem) => {
      queryClient.setQueryData(['items'], (old: unknown[]) => [...(old ?? []), newItem])
      queryClient.invalidateQueries({ queryKey: ['items'] })
    },
  })
}
