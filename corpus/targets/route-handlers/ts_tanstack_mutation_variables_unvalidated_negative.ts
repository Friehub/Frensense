// SAFE: Mutation variables are validated with zod before being sent to the server

import { useMutation } from '@tanstack/react-query'
import { z } from 'zod'

const CreateItemSchema = z.object({
  name: z.string().min(1).max(200),
  price: z.number().positive(),
})

type CreateItemInput = z.infer<typeof CreateItemSchema>

export function useCreateItem() {
  return useMutation({
    mutationFn: async (raw: unknown) => {
      const data = CreateItemSchema.parse(raw)
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify(data),
        headers: { 'Content-Type': 'application/json' },
      })
      return res.json()
    },
  })
}
