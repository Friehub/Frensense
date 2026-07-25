// [frensense]
// observation: `queryClient.prefetchQuery` is called with `staleTime: 0` (default), so the prefetched data becomes stale immediately and every component mount triggers a full refetch
// impact: wasteful network requests — the prefetch never provides any benefit because the data is always stale by the time the component mounts, doubling the request count instead of reducing it
// improvement: set an appropriate `staleTime` on prefetch so the data remains fresh until the component is likely to mount, or use `prefetchInfiniteQuery` for paginated data
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function prefetchUser(userId: string) {
  return queryClient.prefetchQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    staleTime: 0,
  })
}
