// SAFE: Mutation is called with validated data from a controlled form

import { useMutation } from '@tanstack/react-query'
import { z } from 'zod'

const CreateItemSchema = z.object({
  name: z.string().min(1).max(200),
  price: z.number().positive(),
})

type CreateItemInput = z.infer<typeof CreateItemSchema>

export function useCreateItem() {
  return useMutation({
    mutationFn: async (data: CreateItemInput) => {
      const validated = CreateItemSchema.parse(data)
      const res = await fetch('/api/items', {
        method: 'POST',
        body: JSON.stringify(validated),
        headers: { 'Content-Type': 'application/json' },
      })
      return res.json()
    },
  })
}
