// SAFE: search params are parsed via a zod schema that validates the shape and values before use

import { useQuery } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'
import { z } from 'zod'

const FilterSchema = z.object({
  status: z.enum(['active', 'pending', 'completed']).default('active'),
  page: z.coerce.number().int().positive().default(1),
})

export function useFilteredItems() {
  const rawParams = useSearchParams({ from: '/items' })
  const safeParams = FilterSchema.parse(rawParams)

  return useQuery({
    queryKey: ['items', safeParams],
    queryFn: () => fetch(`/api/items?status=${safeParams.status}&page=${safeParams.page}`).then((r) => r.json()),
  })
}
