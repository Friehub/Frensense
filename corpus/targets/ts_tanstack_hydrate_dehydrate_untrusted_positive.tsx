// [frensense]
// observation: `hydrate` is called with data from an untrusted source (e.g., URL params, WebSocket message, or third-party embed) without validation, injecting arbitrary data into the query cache
// impact: cache injection — an attacker can craft dehydrated state containing malicious payloads (XSS via rendered data, manipulated prices, or poisoned user profiles) that are immediately available in the query cache
// improvement: validate dehydrated state against a schema before calling `hydrate`, or only hydrate from trusted server-rendered data
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium

import { Hydrate, QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { useSearchParams } from '@tanstack/react-router'

const queryClient = new QueryClient()

export function App() {
  const searchParams = useSearchParams({ from: '/app' })
  const dehydratedState = searchParams['__dehydrated'] as unknown

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
