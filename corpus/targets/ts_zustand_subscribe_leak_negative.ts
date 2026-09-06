// SAFE: The unsubscribe function returned by subscribe() is stored and called in the useEffect cleanup, preventing memory leaks

import { useEffect } from 'react';
import { useNotificationStore } from './notificationStore';

export function NotificationBadge() {
  const unreadCount = useNotificationStore((s) => s.unreadCount);

  useEffect(() => {
    const unsub = useNotificationStore.subscribe((state, prev) => {
      if (state.unreadCount > prev.unreadCount) {
        console.log('New notification!');
      }
    });
    return () => unsub();
  }, []);

  return <span className="badge">{unreadCount}</span>;
}
