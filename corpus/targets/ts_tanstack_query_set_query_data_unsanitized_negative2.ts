// SAFE: Only server responses are written to the cache, never user input directly

import { useQuery, useQueryClient } from '@tanstack/react-query'

interface UserData {
  name: string
  role: string
}

export function useUserProfile(userId: string) {
  const queryClient = useQueryClient()

  const query = useQuery<UserData>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  return query
}
