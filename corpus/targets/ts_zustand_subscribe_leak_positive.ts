// [frensense]
// observation: `subscribe` listener is registered on the Zustand store but never cleaned up via the returned unsubscribe function, creating a memory leak.
// impact: Every component mount adds a new subscription. Over time these accumulate, causing memory pressure and stale closures that keep large objects from being GC'd.
// improvement: Store the unsubscribe function returned by `subscribe()` and call it in the component cleanup (e.g., useEffect return).

import { useEffect } from 'react';
import { useNotificationStore } from './notificationStore';

export function NotificationBadge() {
  const unreadCount = useNotificationStore((s) => s.unreadCount);

  useEffect(() => {
    useNotificationStore.subscribe((state, prev) => {
      if (state.unreadCount > prev.unreadCount) {
        console.log('New notification!');
      }
    });
  }, []);

  return <span className="badge">{unreadCount}</span>;
}
