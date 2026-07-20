// [frensense]
// observation: A client-side dark mode toggle controls visibility of content that should be hidden from certain users (e.g., premium features, moderation actions, admin panels). The toggle uses CSS-only approach (`dark:` classes or `.dark` selector), but the hidden content is still present in the DOM and visible via stylesheet inspection, direct DOM access, or print/screenshot tools.
// impact: Users who toggle dark mode can reveal UI elements that were intended to be hidden based on access level, subscription tier, or content moderation status. Since the content is always in the DOM, any user can inspect the element, view the text, or access the functionality by toggling the class on the HTML element.
// improvement: Never use CSS dark mode to hide access-controlled content. Use server-side authorization to conditionally render content. CSS dark mode should only control visual theming, not access visibility.

'use client';

import { useState, useEffect } from 'react';

export function PremiumContent({ isPremium }: { isPremium: boolean }) {
  const [darkMode, setDarkMode] = useState(false);

  useEffect(() => {
    document.documentElement.classList.toggle('dark', darkMode);
  }, [darkMode]);

  return (
    <div>
      <button onClick={() => setDarkMode(!darkMode)}>
        Toggle {darkMode ? 'Light' : 'Dark'} Mode
      </button>

      <div className={isPremium ? '' : 'dark:hidden'}>
        <h2>Premium Exclusive Content</h2>
        <p>This content should only be visible to premium users.</p>
      </div>
    </div>
  );
}
