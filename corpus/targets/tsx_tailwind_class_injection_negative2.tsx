// SAFE: User input is sanitized by stripping disallowed characters and prefix-matching against a known allowlist of class prefixes

'use client';

import { useSearchParams } from 'next/navigation';

const ALLOWED_PREFIXES = ['bg-', 'text-', 'border-', 'p-', 'm-', 'gap-', 'shadow-', 'rounded-'];

function sanitizeClass(input: string): string {
  const classes = input.split(/\s+/).filter(Boolean);
  const safe = classes.filter((cls) => ALLOWED_PREFIXES.some((prefix) => cls.startsWith(prefix)));
  return safe.join(' ');
}

export function UserProfileCard({ name }: { name: string }) {
  const searchParams = useSearchParams();
  const rawExtra = searchParams.get('class') ?? '';
  const safeExtra = sanitizeClass(rawExtra);

  return (
    <div className={`p-4 border rounded shadow ${safeExtra}`}>
      <h2 className="text-lg font-bold">{name}</h2>
      <p>Welcome to your profile</p>
    </div>
  );
}
