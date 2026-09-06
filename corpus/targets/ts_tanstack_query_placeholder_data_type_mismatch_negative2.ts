// SAFE: Optional chaining protects against accessing missing placeholder fields

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
  return useQuery<Partial<User>>({
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
    placeholderData: { id: '', name: '' },
  })
}
