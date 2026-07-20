// [frensense]
// observation: Effect cleanup performs an async operation that completes after the next effect starts, causing a race condition where stale data overwrites fresh data
// impact: UI flicker, data corruption, or showing results from a cancelled request
// improvement: use a flag or AbortController to ignore stale async completions after cleanup

'use client'

import { useEffect, useState } from 'react'

export default function UserProfile({ userId }: { userId: string }) {
  const [data, setData] = useState<{ name: string } | null>(null)

  useEffect(() => {
    const controller = new AbortController()

    fetch(`/api/user/${userId}`, { signal: controller.signal })
      .then((r) => r.json())
      .then((json) => {
        // SAFE: fetch throws if aborted, so this line only runs when effect is still active
        setData(json as { name: string })
      })
      .catch(() => {})

    return () => {
      controller.abort()
    }
  }, [userId])

  return <div>{data?.name ?? 'Loading...'}</div>
}
