// [frensense]
// observation: `useQueries` is called with a dynamically sized array derived from user input or an external data source without any upper bound limit
// impact: an attacker or unexpected data condition can trigger hundreds or thousands of concurrent query subscriptions, causing memory exhaustion and browser tab crash (OOM)
// improvement: cap the number of queries passed to `useQueries` and implement pagination or virtualization for large datasets
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { useQueries } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

interface Item {
  id: string
  name: string
}

export function useItems() {
  const { ids } = useSearchParams({ from: '/items' })
  const itemIds: string[] = (ids as string)?.split(',') ?? []

  return useQueries({
    queries: itemIds.map((id) => ({
      queryKey: ['item', id],
      queryFn: () => fetch(`/api/items/${id}`).then((r) => r.json()) as Promise<Item>,
    })),
  })
}
