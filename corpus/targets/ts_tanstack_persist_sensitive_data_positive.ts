// [frensense]
// observation: `persistQueryClient` stores the entire query cache in localStorage, including query data that contains PII (email, SSN, etc.)
// impact: sensitive user data persists in browser storage and can be exfiltrated by another script or accessed via browser dev tools
// improvement: implement a custom `serializer` in `persistQueryClient` that strips sensitive fields, or set `maxAge` to a short TTL
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { QueryClient } from '@tanstack/react-query'
import { persistQueryClient } from '@tanstack/react-query-persist-client'
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister'

const queryClient = new QueryClient()

const localStoragePersister = createSyncStoragePersister({
  storage: window.localStorage,
})

persistQueryClient({
  queryClient,
  persister: localStoragePersister,
  maxAge: 1000 * 60 * 60 * 24,
})

export { queryClient }
