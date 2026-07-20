// [frensense]
// observation: `queryClient.setQueryData` is called with user-controlled data (from URL params, form input, or WebSocket messages) without sanitization or validation
// impact: cache poisoning — an attacker can inject arbitrary data into the query cache, which may contain XSS payloads, manipulated prices, or other malicious content that gets rendered in the UI
// improvement: validate and sanitize user-controlled data against a schema before writing it to the query cache

import { useQueryClient } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

interface UserData {
  name: string
  role: string
}

export function useCacheUserData() {
  const queryClient = useQueryClient()
  const searchParams = useSearchParams({ from: '/user' })

  const raw = searchParams as unknown as UserData

  queryClient.setQueryData(['user', 'profile'], raw)
}
