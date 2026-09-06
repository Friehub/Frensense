// SAFE: Subscribe is called with a selector and equality function, and the listener is removed on component unmount via cleanup

import { useEffect, useRef } from 'react';
import { useNotificationStore } from './notificationStore';

export function NotificationBadge() {
  const unreadCount = useNotificationStore((s) => s.unreadCount);
  const unsubRef = useRef<(() => void) | null>(null);

  useEffect(() => {
    unsubRef.current = useNotificationStore.subscribe(
      (state) => state.unreadCount,
      (current, previous) => {
        if (current > previous) {
          console.log('New notification!');
        }
      }
    );
    return () => unsubRef.current?.();
  }, []);

  return <span className="badge">{unreadCount}</span>;
}
