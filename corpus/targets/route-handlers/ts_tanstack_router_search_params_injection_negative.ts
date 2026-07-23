// SAFE: search params are validated against an allowlist before being used in the query

import { useQuery } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

const ALLOWED_STATUSES = ['active', 'pending', 'completed', 'cancelled'] as const

export function useFilteredItems() {
  const { status } = useSearchParams({ from: '/items' })
  const safeStatus = ALLOWED_STATUSES.includes(status as typeof ALLOWED_STATUSES[number]) ? status : 'active'

  return useQuery({
    queryKey: ['items', { status: safeStatus }],
    queryFn: () => fetch(`/api/items?status=${safeStatus}`).then((r) => r.json()),
  })
}
