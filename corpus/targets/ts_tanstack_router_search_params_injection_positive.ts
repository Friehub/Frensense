// [frensense]
// observation: search params from the URL are used directly in a TanStack Query queryFn without validation, allowing an attacker to inject malicious values
// impact: search param injection — an attacker can craft a URL with manipulated search params (e.g., `?status=admin` or `?userId=../ secret`) to access unauthorized data or perform injection attacks
// improvement: validate and sanitize search params against an allowlist or schema before using them in queries

import { useQuery } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

export function useFilteredItems() {
  const { status } = useSearchParams({ from: '/items' })

  return useQuery({
    queryKey: ['items', { status }],
    queryFn: () => fetch(`/api/items?status=${status}`).then((r) => r.json()),
  })
}
