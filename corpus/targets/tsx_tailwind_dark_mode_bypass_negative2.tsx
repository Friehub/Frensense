// SAFE: Dark mode toggles visual themes only, never access control. Authorized content is rendered server-side with proper auth checks.

'use client';

import { useState } from 'react';

export function SettingsPanel({ isAdmin }: { isAdmin: boolean }) {
  const [theme, setTheme] = useState<'light' | 'dark'>('light');

  return (
    <div data-theme={theme}>
      <button onClick={() => setTheme(theme === 'dark' ? 'light' : 'dark')}>
        Toggle {theme === 'dark' ? 'Light' : 'Dark'} Mode
      </button>

      <h2>Settings</h2>
      <p>Account preferences</p>

      {isAdmin && (
        <div className={theme === 'dark' ? 'bg-gray-800 text-white' : 'bg-gray-100 text-black'}>
          <h3>Admin Controls</h3>
          <button>Delete All Users</button>
        </div>
      )}
    </div>
  );
}
