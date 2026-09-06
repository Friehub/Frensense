// [frensense]
// observation: Effect cleanup performs an async operation that completes after the next effect starts, causing a race condition where stale data overwrites fresh data
// impact: UI flicker, data corruption, or showing results from a cancelled request
// improvement: use a flag or AbortController to ignore stale async completions after cleanup
// cwe: CWE-362
// cvss: 7.0
// owasp: 
// severity: High

'use client'

import { useEffect, useState } from 'react'

export default function UserProfile({ userId }: { userId: string }) {
  const [data, setData] = useState<{ name: string } | null>(null)

  useEffect(() => {
    fetch(`/api/user/${userId}`)
      .then((r) => r.json())
      .then((json) => setData(json as { name: string }))

    return () => {
      setTimeout(() => {
        console.log('cleanup done')
      }, 1000)
    }
  }, [userId])

  return <div>{data?.name ?? 'Loading...'}</div>
}
