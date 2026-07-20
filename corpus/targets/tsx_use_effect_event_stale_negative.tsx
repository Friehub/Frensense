// SAFE: useEffectEvent uses a ref mapped from props to always read the latest value

'use client'

import { useEffect, useRef, useEffectEvent } from 'react'

export default function NotificationSender({ userId }: { userId: string }) {
  const userIdRef = useRef(userId)
  userIdRef.current = userId

  const settingsRef = useRef<{ theme: string }>({ theme: 'light' })

  const sendNotification = useEffectEvent(async (message: string) => {
    const theme = settingsRef.current.theme
    const currentUserId = userIdRef.current

    await fetch('/api/notify', {
      method: 'POST',
      body: JSON.stringify({ userId: currentUserId, message, theme }),
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
