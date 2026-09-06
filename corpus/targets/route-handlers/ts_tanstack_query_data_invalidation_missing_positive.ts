// [frensense]
// observation: a mutation creates a new resource but never invalidates or updates the list query, so the list view shows stale data
// impact: users do not see newly created items until they manually refresh the page, causing confusion about whether the operation succeeded
// improvement: call `queryClient.invalidateQueries` with the related list query key after a successful mutation
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { useMutation, useQuery } from '@tanstack/react-query'

export function useItems() {
  return useQuery({
    queryKey: ['items'],
    queryFn: () => fetch('/api/items').then((r) => r.json()),
  })
}

export function useCreateItem() {
  return useMutation({
    mutationKey: ['items', 'create'],
    mutationFn: async (name: string) => {
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify({ name }),
      })
      return res.json()
    },
  })
}
