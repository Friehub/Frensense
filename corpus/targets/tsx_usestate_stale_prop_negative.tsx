// SAFE: Local state is synced with prop changes via useEffect, ensuring displayName stays current when user prop updates.

import { useState, useEffect } from 'react';

interface User {
  id: string;
  name: string;
  email: string;
}

export function UserProfile({ user }: { user: User }) {
  const [displayName, setDisplayName] = useState(user.name);

  useEffect(() => {
    setDisplayName(user.name);
  }, [user.name]);

  const handleEdit = (newName: string) => {
    setDisplayName(newName);
  };

  return (
    <div>
      <h1>{displayName}</h1>
      <p>{user.email}</p>
      <input value={displayName} onChange={(e) => handleEdit(e.target.value)} />
    </div>
  );
}
