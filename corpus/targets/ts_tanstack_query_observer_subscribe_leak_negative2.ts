// SAFE: Using useQuery hook instead of manual QueryObserver to avoid subscription management

import { useQuery } from '@tanstack/react-query'

export function useUser(userId: string) {
  return useQuery({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })
}
