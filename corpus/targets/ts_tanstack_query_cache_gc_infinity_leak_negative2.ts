// SAFE: Manual cache pruning removes old entries when they are no longer needed

import { useQuery, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function useDocument(docId: string) {
  const [prevId, setPrevId] = useState<string | null>(null)

  const query = useQuery({
    queryKey: ['document', docId],
    queryFn: () => fetch(`/api/documents/${docId}`).then((r) => r.json()),
    gcTime: Infinity,
  })

  if (prevId && prevId !== docId) {
    queryClient.removeQueries({ queryKey: ['document', prevId] })
    setPrevId(docId)
  }

  return query
}

function useState<T>(init: T): [T, (v: T) => void] {
  const [state, setState] = (window as any).__useState?.(init) ?? [init, () => {}]
  return [state, setState]
}
