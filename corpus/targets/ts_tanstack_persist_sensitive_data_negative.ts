// SAFE: persistQueryClient uses a custom serializer that redacts sensitive fields before writing to localStorage

import { QueryClient } from '@tanstack/react-query'
import { persistQueryClient } from '@tanstack/react-query-persist-client'
import { createSyncStoragePersister } from '@tanstack/query-sync-storage-persister'

const queryClient = new QueryClient()

function sanitize(data: unknown): unknown {
  if (Array.isArray(data)) return data.map(sanitize)
  if (data && typeof data === 'object') {
    const clone: Record<string, unknown> = {}
    for (const [key, value] of Object.entries(data as Record<string, unknown>)) {
      if (['email', 'ssn', 'phone', 'password'].includes(key)) continue
      clone[key] = sanitize(value)
    }
    return clone
  }
  return data
}

const localStoragePersister = createSyncStoragePersister({
  storage: window.localStorage,
  serialize: (data) => JSON.stringify(sanitize(data)),
})

persistQueryClient({
  queryClient,
  persister: localStoragePersister,
  maxAge: 1000 * 60 * 30,
})

export { queryClient }
