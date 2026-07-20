// [frensense]
// observation: dynamically generated query keys (e.g., per-user document queries) use `gcTime: Infinity`, so the cache grows unbounded as new queries are created
// impact: memory exhaustion on the client — each unique query key creates a permanent cache entry that is never garbage-collected, leading to browser tab OOM crashes after extended use
// improvement: set a finite `gcTime` (default 5 minutes) or use `maxAge` in persistence, or manually prune old cache entries with `queryClient.removeQueries`

import { useQuery, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function useDocument(docId: string) {
  return useQuery({
    queryKey: ['document', docId],
    queryFn: () => fetch(`/api/documents/${docId}`).then((r) => r.json()),
    gcTime: Infinity,
  })
}

export function useAllDocuments(docIds: string[]) {
  const results = docIds.map((id) => ({
    queryKey: ['document', id],
    queryFn: () => fetch(`/api/documents/${id}`).then((r) => r.json()),
    gcTime: Infinity,
  }))
  return results
}
