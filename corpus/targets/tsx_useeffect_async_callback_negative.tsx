// SAFE: async logic is defined and called inside the effect; AbortController handles cleanup

import { useEffect, useState } from 'react';

export function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ name: string } | null>(null);

  useEffect(() => {
    const controller = new AbortController();
    async function load() {
      const response = await fetch(`/api/users/${userId}`, { signal: controller.signal });
      const data = await response.json();
      setUser(data);
    }
    load();
    return () => controller.abort();
  }, [userId]);

  return <div>{user?.name}</div>;
}
