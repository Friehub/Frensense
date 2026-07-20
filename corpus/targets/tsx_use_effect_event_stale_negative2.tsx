// SAFE: useEffectEvent reads props via a stable ref that is kept in sync, avoiding stale closure issues

'use client'

import { useEffect, useRef, useEffectEvent } from 'react'

export default function NotificationSender({ userId }: { userId: string }) {
  const latestUserId = useRef(userId)
  latestUserId.current = userId

  const sendNotification = useEffectEvent(async (message: string) => {
    await fetch('/api/notify', {
      method: 'POST',
      body: JSON.stringify({ userId: latestUserId.current, message }),
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
