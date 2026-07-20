// SAFE: the cursor is validated (type-checked and length-limited) before being used in the query URL

import { useSuspenseInfiniteQuery } from '@tanstack/react-query'

interface PageResponse {
  items: string[]
  nextCursor: string | null
}

function sanitizeCursor(cursor: unknown): string {
  if (typeof cursor !== 'string') return ''
  if (cursor.length > 128) return ''
  if (!/^[a-zA-Z0-9_\-=]+$/.test(cursor)) return ''
  return cursor
}

export function useItems() {
  return useSuspenseInfiniteQuery<PageResponse>({
    queryKey: ['items'],
    initialPageParam: null as string | null,
    queryFn: async ({ pageParam }) => {
      const cursor = sanitizeCursor(pageParam)
      const res = await fetch(`/api/items?cursor=${cursor}`)
      return res.json()
    },
    getNextPageParam: (lastPage) => lastPage.nextCursor,
  })
}
