// SAFE: select returns a transformed DTO that only includes fields safe for the component

import { useQuery } from '@tanstack/react-query'

interface RawUser {
  id: string
  name: string
  email: string
  passwordHash: string
  ssn: string
}

interface SafeProfile {
  displayName: string
  contactEmail: string
}

export function useUserProfile(id: string) {
  return useQuery({
    queryKey: ['user', id],
    queryFn: () => fetch(`/api/users/${id}`).then((r) => r.json()),
    select: (data: RawUser): SafeProfile => ({
      displayName: data.name,
      contactEmail: data.email,
    }),
  })
}
