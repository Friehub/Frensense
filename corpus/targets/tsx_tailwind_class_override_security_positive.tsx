// [frensense]
// observation: User-controlled input is used to add Tailwind classes that include `!important` overrides (e.g., `!hidden`, `!block`, `!flex`). This allows an attacker to override security-critical styling such as overlay z-indexing, modal visibility, or warning banners that depend on class order specificity.
// impact: An attacker can hide security indicators, warning banners, or authentication overlays by injecting `!hidden` or `!pointer-events-none`. This enables clickjacking, phishing, or bypassing security warnings without the user's knowledge.
// improvement: Never allow user-controlled class strings to include `!important` variants. Validate or strip `!` prefixed classes from user input, or use an allowlist of safe classes.

'use client';

import { useSearchParams } from 'next/navigation';

export function SecurityBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const extra = searchParams.get('class') ?? '';

  return (
    <div className="relative">
      <div className={`fixed inset-0 z-50 flex items-center justify-center bg-black/50`}>
        <div className={`bg-white p-6 rounded-lg shadow-xl ${extra}`}>
          <h2 className="text-xl font-bold text-red-600">Security Warning</h2>
          <p>{message}</p>
        </div>
      </div>
    </div>
  );
}
