// SAFE: persistQueryClient uses sessionStorage instead of localStorage and sets a short maxAge

import { QueryClient } from '@tanstack/react-query'
import { persistQueryClient } from '@tanstack/react-query-persist-client'
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister'

const queryClient = new QueryClient()

const sessionStoragePersister = createSyncStoragePersister({
  storage: window.sessionStorage,
})

persistQueryClient({
  queryClient,
  persister: sessionStoragePersister,
  maxAge: 1000 * 60 * 5,
})

export { queryClient }
