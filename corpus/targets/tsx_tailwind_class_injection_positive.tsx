// [frensense]
// observation: User-controlled input is interpolated directly into the `className` prop, allowing CSS injection via Tailwind class names. An attacker can inject class strings that alter layout, hide elements, or exfiltrate data via CSS-based tracking techniques.
// impact: An attacker can inject arbitrary Tailwind utility classes (e.g., `bg-[url(https://attacker.com/steal)]`, `text-transparent`, `hidden`, `pointer-events-none`) to visually manipulate the UI, hide security indicators, or exfiltrate data via CSS background-image URLs that trigger on hover/visibility. CSS injection can also be used for phishing by styling fake UI elements.
// improvement: Use a predefined set of allowed class name mappings instead of passing user input to className. If dynamic classes are necessary, validate against an allowlist.

'use client';

import { useSearchParams } from 'next/navigation';

export function UserProfileCard({ name }: { name: string }) {
  const searchParams = useSearchParams();
  const extraClass = searchParams.get('class') ?? '';

  return (
    <div className={`p-4 border rounded shadow ${extraClass}`}>
      <h2 className="text-lg font-bold">{name}</h2>
      <p>Welcome to your profile</p>
    </div>
  );
}
