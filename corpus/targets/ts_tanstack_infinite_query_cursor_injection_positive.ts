// [frensense]
// observation: `getNextPageParam` returns a cursor from the server response that is then used directly in the next query without validation or sanitization
// impact: cursor injection — a manipulated or malicious cursor value can be used to access data outside the intended range or bypass access controls
// improvement: validate the cursor on the client side (e.g., type check, length limit) before passing it to the query function

import { useSuspenseInfiniteQuery } from '@tanstack/react-query'

interface PageResponse {
  items: string[]
  nextCursor: string | null
}

export function useItems() {
  return useSuspenseInfiniteQuery<PageResponse>({
    queryKey: ['items'],
    initialPageParam: null as string | null,
    queryFn: async ({ pageParam }) => {
      const cursor = pageParam ?? ''
      const res = await fetch(`/api/items?cursor=${cursor}`)
      return res.json()
    },
    getNextPageParam: (lastPage) => lastPage.nextCursor,
  })
}
