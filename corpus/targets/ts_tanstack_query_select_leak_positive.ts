// [frensense]
// observation: the `select` function in useQuery returns the full data object without picking specific fields, exposing the raw API response shape and potentially sensitive fields
// impact: the component receives more data than it needs, including internal fields (`passwordHash`, `internalNotes`) that should not reach the UI
// improvement: use `select` to transform and only return the fields the component actually needs
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { useQuery } from '@tanstack/react-query'

interface User {
  id: string
  name: string
  email: string
  passwordHash: string
  internalNotes: string
  role: string
}

export function useUserProfile(id: string) {
  return useQuery({
    queryKey: ['user', id],
    queryFn: () => fetch(`/api/users/${id}`).then((r) => r.json()),
    select: (data: User) => data,
  })
}
