// SAFE: untrusted dehydrated state is validated against a schema before hydrate

import { Hydrate, QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { z } from 'zod'

const DehydratedStateSchema = z.object({
  mutations: z.array(z.unknown()),
  queries: z.array(z.object({
    queryKey: z.array(z.unknown()),
    state: z.object({
      data: z.unknown(),
      dataUpdatedAt: z.number(),
      status: z.enum(['pending', 'success', 'error']),
    }),
  })),
})

const queryClient = new QueryClient()

export function App({ dehydratedState }: { dehydratedState: unknown }) {
  const safe = DehydratedStateSchema.parse(dehydratedState)

  return (
    <QueryClientProvider client={queryClient}>
      <Hydrate state={safe}>
        <MainContent />
      </Hydrate>
    </QueryClientProvider>
  )
}

function MainContent() {
  return <div>App content</div>
}
