// [frensense]
// observation: User-controlled input is interpolated into a Tailwind arbitrary value (`[color:${param}]`). This allows an attacker to inject arbitrary CSS properties via the bracket syntax, potentially exfiltrating data via CSS selectors (e.g., `input[value^=a]` background URLs).
// impact: An attacker can inject CSS property-value pairs that exfiltrate sensitive data (CSRF tokens, form values) through CSS selector-based background-image URL callbacks. CSS injection can also be used to deface the page or perform phishing by overriding styles.
// improvement: Avoid using user input in Tailwind arbitrary values. Use a mapping from user-provided enum values to predefined Tailwind classes instead.

'use client';

import { useSearchParams } from 'next/navigation';

export function ThemeCard({ children }: { children: React.ReactNode }) {
  const searchParams = useSearchParams();
  const accentColor = searchParams.get('accent') ?? '#3b82f6';

  return (
    <div className={`p-4 border rounded shadow`}>
      <div className={`text-[color:${accentColor}] font-bold`}>
        User Profile
      </div>
      {children}
    </div>
  );
}
