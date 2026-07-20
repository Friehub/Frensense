// SAFE: Query count is capped to prevent unbounded memory allocation

import { useQueries } from '@tanstack/react-query'

const MAX_QUERIES = 50

interface Item {
  id: string
  name: string
}

export function useItems(itemIds: string[]) {
  const capped = itemIds.slice(0, MAX_QUERIES)

  return useQueries({
    queries: capped.map((id) => ({
      queryKey: ['item', id],
      queryFn: () => fetch(`/api/items/${id}`).then((r) => r.json()) as Promise<Item>,
    })),
  })
}
