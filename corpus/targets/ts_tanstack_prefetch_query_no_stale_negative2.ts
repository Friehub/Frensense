// SAFE: Using setQueryData with the prefetched data avoids default staleTime=0 issue

import { QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export async function prefetchUser(userId: string) {
  const data = await fetch(`/api/users/${userId}`).then((r) => r.json())

  queryClient.setQueryData(['user', userId], data)
}
