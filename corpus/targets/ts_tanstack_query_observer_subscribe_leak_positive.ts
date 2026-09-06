// [frensense]
// observation: a manual `QueryObserver` is created and `subscribe()` is called, but the returned unsubscribe function is never invoked on component unmount or effect cleanup
// impact: the observer callback continues to fire after the component unmounts, causing memory leaks (the observer and its closure are retained) and potential state updates on unmounted components
// improvement: call the unsubscribe function returned by `subscribe()` in a cleanup function (e.g., useEffect return or AbortSignal)
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { QueryObserver, QueryClient } from '@tanstack/react-query'

const queryClient = new QueryClient()

export function observeUser(userId: string, onData: (data: unknown) => void) {
  const observer = new QueryObserver(queryClient, {
    queryKey: ['user', userId],
    queryFn: () => fetch(`/api/users/${userId}`).then((r) => r.json()),
  })

  observer.subscribe((result) => {
    onData(result.data)
  })
}
