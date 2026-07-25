// [frensense]
// observation: Sensitive content is visually hidden in light mode using Tailwind's `dark:` variant (e.g., `hidden dark:block` or `dark:hidden` inverted). This provides only CSS-based access control — the content remains in the DOM and is trivially revealed by toggling dark mode via browser DevTools or URL parameters.
// impact: An attacker can reveal sensitive information (admin panels, premium content, moderation UI, PII) by toggling the browser's color scheme or injecting a `dark` class on the `<html>` element. Since the content is always in the DOM, it is also exposed to screen readers, search engines, and DOM inspection tools.
// improvement: Never use CSS dark mode to control access to sensitive content. Always use server-side authorization to conditionally render protected content.
// cwe: CWE-200
// cvss: 4.3
// owasp: 
// severity: Low

'use client';

import { useState, useEffect } from 'react';

export function AdminControls({ role }: { role: string }) {
  const [dark, setDark] = useState(false);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', dark);
  }, [dark]);

  return (
    <div>
      <button onClick={() => setDark((v) => !v)}>
        {dark ? 'Light' : 'Dark'} Mode
      </button>

      <div className="hidden dark:block">
        <h2>Admin Panel</h2>
        <p>Delete users, view logs, manage billing</p>
        <button>Delete All Users</button>
      </div>
    </div>
  );
}
