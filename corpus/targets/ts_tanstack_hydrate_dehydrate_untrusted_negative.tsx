// SAFE: Hydrate only receives data from a trusted server-side source

import { Hydrate, QueryClient, QueryClientProvider, dehydrate } from '@tanstack/react-query'

const queryClient = new QueryClient()

export async function App() {
  await queryClient.prefetchQuery({
    queryKey: ['items'],
    queryFn: () => fetch('/api/items').then((r) => r.json()),
  })

  const dehydratedState = dehydrate(queryClient)

  return (
    <QueryClientProvider client={queryClient}>
      <Hydrate state={dehydratedState}>
        <MainContent />
      </Hydrate>
    </QueryClientProvider>
  )
}

function MainContent() {
  return <div>App content</div>
}
