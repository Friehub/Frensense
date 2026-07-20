// SAFE: The unsubscribe function returned by subscribe is stored and called to clean up

import { QueryObserver, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function observeUser(userId: string, onData: (data: unknown) => void, signal?: AbortSignal) {
  const observer = new QueryObserver(queryClient, {
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  const unsubscribe = observer.subscribe((result) => {
    onData(result.data)
  })

  signal?.addEventListener('abort', () => {
    unsubscribe()
  })
}
