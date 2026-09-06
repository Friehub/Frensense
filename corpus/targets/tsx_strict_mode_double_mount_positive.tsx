// [frensense]
// observation: `StrictMode` double-mounts components in development, causing effects that fetch data to fire twice and create duplicate API requests
// impact: duplicate API calls cause double billing, duplicate side effects (e.g., email sends), or rate limit triggering during development
// improvement: use an idempotency key or deduplication logic in the effect

'use client'

import { StrictMode, useEffect, useState } from 'react'

function CheckoutPage() {
  const [charged, setCharged] = useState(false)

  useEffect(() => {
    fetch('/api/charge', { method: 'POST', body: JSON.stringify({ amount: 100 }) })
    setCharged(true)
  }, [])

  return <div>{charged ? 'Charged!' : 'Processing...'}</div>
}

export default function App() {
  return (
    <StrictMode>
      <CheckoutPage />
    </StrictMode>
  )
}
