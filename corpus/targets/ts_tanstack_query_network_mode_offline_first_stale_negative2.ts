// SAFE: offlineFirst is acceptable here because staleTime ensures periodic refetches

import { useQuery } from '@tanstack/react-query'

export function useUserProfile(userId: string) {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    networkMode: 'offlineFirst',
    staleTime: 30_000,
  })
}
