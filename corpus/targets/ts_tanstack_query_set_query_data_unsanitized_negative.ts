// SAFE: User-controlled data is validated against a schema before being written to the cache

import { useQueryClient } from '@tanstack/react-query'
import { z } from 'zod'

const UserDataSchema = z.object({
  name: z.string().min(1).max(100),
  role: z.enum(['admin', 'user', 'viewer']),
})

interface UserData {
  name: string
  role: string
}

export function useCacheUserData(raw: unknown) {
  const queryClient = useQueryClient()

  const parsed = UserDataSchema.parse(raw)

  queryClient.setQueryData(['user', 'profile'], parsed)
}
