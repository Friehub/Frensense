// [frensense]
// observation: `useEffectEvent` captures a ref that becomes stale, causing the event callback to reference old data instead of the latest value
// impact: the wrong callback is invoked, leading to data loss — e.g., sending a notification to the wrong recipient or using outdated state
// improvement: use a ref or ensure the useEffectEvent callback reads the latest value via a stable reference

'use client'

import { useEffect, useRef, useEffectEvent } from 'react'

export default function NotificationSender({ userId }: { userId: string }) {
  const settingsRef = useRef<{ theme: string }>({ theme: 'light' })

  const sendNotification = useEffectEvent(async (message: string) => {
    const theme = settingsRef.current.theme

    await fetch('/api/notify', {
      method: 'POST',
      body: JSON.stringify({ userId, message, theme }),
    })
  })

  useEffect(() => {
    const interval = setInterval(() => {
      sendNotification('Daily reminder')
    }, 60000)
    return () => clearInterval(interval)
  }, [])

  return <p>Notifications active for {userId}</p>
}
