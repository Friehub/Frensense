// [frensense]
// observation: `networkMode: 'offlineFirst'` is set on a query, so when the network is actually available, the query may return a stale cached value without ever attempting to refetch from the server
// impact: users see stale data because the query does not refetch in online mode — it serves the cached response as if it were fresh, masking data changes on the server
// improvement: use `networkMode: 'online'` (default) for data that must be fresh, or only use `offlineFirst` for truly optional/cached data and combine it with `staleTime` to control freshness

import { useQuery } from '@tanstack/react-query'

export function useUserProfile(userId: string) {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    networkMode: 'offlineFirst',
    staleTime: 0,
  })
}
