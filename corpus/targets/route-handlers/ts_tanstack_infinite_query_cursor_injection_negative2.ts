// SAFE: cursor is validated server-side via a signed token, client only validates type and length

import { useSuspenseInfiniteQuery } from '@tanstack/react-query'

interface PageResponse {
  items: string[]
  nextCursor: string | null
}

function isValidCursor(cursor: unknown): cursor is string {
  return typeof cursor === 'string' && cursor.length > 0 && cursor.length <= 256
}

export function useItems() {
  return useSuspenseInfiniteQuery<PageResponse>({
    queryKey: ['items'],
    initialPageParam: null as string | null,
    queryFn: async ({ pageParam }) => {
      const cursor = isValidCursor(pageParam) ? pageParam : ''
      const res = await fetch(`/api/items?cursor=${encodeURIComponent(cursor)}`)
      if (!res.ok) throw new Error('Failed to fetch')
      return res.json()
    },
    getNextPageParam: (lastPage) => lastPage.nextCursor,
  })
}
