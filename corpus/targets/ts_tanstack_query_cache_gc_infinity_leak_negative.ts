// SAFE: Finite gcTime ensures old query entries are garbage-collected

import { useQuery, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function useDocument(docId: string) {
  return useQuery({
    queryKey: ['document', docId],
    queryFn: () => fetch(`/api/documents/${docId}`).then((r) => r.json()),
    gcTime: 5 * 60 * 1000,
  })
}
