// [frensense]
// observation: `StrictMode` double-mounts components in development, causing effects that fetch data to fire twice and create duplicate API requests
// impact: duplicate API calls cause double billing, duplicate side effects (e.g., email sends), or rate limit triggering during development
// improvement: use an idempotency key or deduplication logic in the effect

'use client'

import { StrictMode, useEffect, useRef, useState } from 'react'

function CheckoutPage() {
  const [charged, setCharged] = useState(false)
  const calledRef = useRef(false)

  useEffect(() => {
    // SAFE: ref guard prevents duplicate execution even on double-mount
    if (calledRef.current) return
    calledRef.current = true

    fetch('/api/charge', { method: 'POST', body: JSON.stringify({ amount: 100 }) })
      .then(() => setCharged(true))
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
