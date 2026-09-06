// [frensense]
// observation: An async function is passed directly as the useEffect callback, which returns a Promise instead of a cleanup function.
// impact: React calls the async function but cannot properly clean it up. If the component unmounts before the async operation completes, setState is called on an unmounted component, causing memory leaks and 'Can't perform a React state update' warnings.
// improvement: Define the async logic inside the effect and call it immediately, handling cleanup with an AbortController or mounted flag.

import { useEffect, useState } from 'react';

export function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ name: string } | null>(null);

  useEffect(async () => {
    const response = await fetch(`/api/users/${userId}`);
    const data = await response.json();
    setUser(data);
  }, [userId]);

  return <div>{user?.name}</div>;
}
