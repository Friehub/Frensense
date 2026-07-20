// [frensense]
// observation: `placeholderData` returns an object with a different shape or type than the actual query data, so consuming code that expects the real shape crashes at runtime
// impact: runtime type error — placeholder data with missing fields or wrong types causes `undefined is not an object` or similar crashes when the UI tries to render nested properties
// improvement: ensure placeholderData matches the exact shape and type of the real query data, or use a type guard before accessing placeholder fields

import { useQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
  address: {
    city: string
    zip: string
  }
}

export function useUser(userId: string) {
  return useQuery<User>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    placeholderData: { id: '', name: '' } as User,
  })
}
