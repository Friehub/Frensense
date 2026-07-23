// [frensense]
// observation: mutation variables come from user input (form data, search params, etc.) without any schema validation, so arbitrary or malformed data is sent to the server
// impact: injection attacks or data corruption — an attacker can send manipulated mutation variables (e.g., setting `role: 'admin'`, injecting SQL via string fields, or sending oversized payloads)
// improvement: validate mutation variables against a schema (e.g., zod) before passing them to `mutate`, or validate inside the `mutationFn`

import { useMutation } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

interface CreateItemInput {
  name: string
  price: number
}

export function useCreateItem() {
  const searchParams = useSearchParams({ from: '/items/create' })

  return useMutation({
    mutationFn: async (data: CreateItemInput) => {
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: { 'Content-Type': 'application/json' },
      })
      return res.json()
    },
  })
}
