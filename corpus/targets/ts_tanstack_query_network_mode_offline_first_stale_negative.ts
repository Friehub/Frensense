// SAFE: networkMode is 'online' (default), so it always fetches fresh data from the server

import { useQuery } from '@tanstack/react-query'

export function useUserProfile(userId: string) {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })
}
