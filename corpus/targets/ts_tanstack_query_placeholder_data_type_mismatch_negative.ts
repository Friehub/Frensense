// SAFE: placeholderData matches the full shape of the expected query data

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
    placeholderData: {
      id: '',
      name: '',
      address: { city: '', zip: '' },
    } as User,
  })
}
