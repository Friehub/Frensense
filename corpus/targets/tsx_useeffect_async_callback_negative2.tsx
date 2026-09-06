// SAFE: uses a mounted flag to avoid setting state on unmounted component

import { useEffect, useState, useRef } from 'react';

export function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ name: string } | null>(null);
  const mountedRef = useRef(true);

  useEffect(() => {
    mountedRef.current = true;
    fetch(`/api/users/${userId}`)
      .then(r => r.json())
      .then(data => {
        if (mountedRef.current) setUser(data);
      });
    return () => { mountedRef.current = false; };
  }, [userId]);

  return <div>{user?.name}</div>;
}
