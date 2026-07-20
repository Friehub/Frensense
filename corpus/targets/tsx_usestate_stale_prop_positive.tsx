// [frensense]
// observation: A React component initializes local state from props using useState, but never updates that state when the props change, causing it to render stale data.
// impact: The component displays out-of-date information after the parent re-renders with new prop values, leading to UI inconsistency and potential data integrity bugs.
// improvement: Either sync state with props via useEffect, derive the value directly without local state, or use a key prop to force remount.

import { useState } from 'react';

interface User {
  id: string;
  name: string;
  email: string;
}

export function UserProfile({ user }: { user: User }) {
  const [displayName, setDisplayName] = useState(user.name);

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
