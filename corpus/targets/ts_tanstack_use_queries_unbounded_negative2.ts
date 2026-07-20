// SAFE: Pagination is used instead of loading all items at once

import { useQuery } from '@tanstack/react-query'
import { useQueries } from '@tanstack/react-query'

interface Item {
  id: string
  name: string
}

export function useItems(itemIds: string[], page: number, pageSize: number = 50) {
  const start = (page - 1) * pageSize
  const end = start + pageSize
  const pageIds = itemIds.slice(start, end)

  return useQueries({
    queries: pageIds.map((id) => ({
      queryKey: ['item', id],
      queryFn: () => fetch(`/api/items/${id}`).then((r) => r.json()) as Promise<Item>,
    })),
  })
}
