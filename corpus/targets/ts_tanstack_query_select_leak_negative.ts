// SAFE: select returns only the fields needed by the component, stripping sensitive internal data

import { useQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
  email: string
  passwordHash: string
  internalNotes: string
  role: string
}

interface UserProfile {
  id: string
  name: string
  email: string
  role: string
}

export function useUserProfile(id: string) {
  return useQuery({
    queryKey: ['user', id],
    queryFn: () => fetch(`/api/users/${id}`).then((r) => r.json()),
    select: (data: User): UserProfile => ({
      id: data.id,
      name: data.name,
      email: data.email,
      role: data.role,
    }),
  })
}
