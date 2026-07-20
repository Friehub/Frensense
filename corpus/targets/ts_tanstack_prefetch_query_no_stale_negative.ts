// SAFE: staleTime on prefetch ensures the data remains fresh until the component mounts

import { QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function prefetchUser(userId: string) {
  return queryClient.prefetchQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    staleTime: 30_000,
  })
}
