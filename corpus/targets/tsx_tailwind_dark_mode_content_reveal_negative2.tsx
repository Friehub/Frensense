// SAFE: Dark mode only controls visual theming styles (colors, backgrounds). Access-restricted content uses proper authorization checks, and dark mode variants are only applied to cosmetic classes like `dark:bg-gray-800` and `dark:text-white`.

'use client';

import { useState } from 'react';

export function AdminControls({ role }: { role: string }) {
  const [theme, setTheme] = useState<'light' | 'dark'>('light');
  const isAdmin = role === 'admin';

  return (
    <div className={theme === 'dark' ? 'bg-gray-900' : 'bg-white'}>
      <button onClick={() => setTheme((v) => (v === 'dark' ? 'light' : 'dark'))}>
        Toggle Theme
      </button>

      <h2 className={theme === 'dark' ? 'text-white' : 'text-black'}>
        Dashboard
      </h2>

      {isAdmin && (
        <div>
          <h3 className={theme === 'dark' ? 'text-gray-100' : 'text-gray-900'}>
            Admin Panel
          </h3>
          <button>Delete All Users</button>
        </div>
      )}
    </div>
  );
}
