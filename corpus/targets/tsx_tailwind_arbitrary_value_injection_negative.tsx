// SAFE: User input for accent color is validated against an allowlist of color names mapped to predefined Tailwind classes. No arbitrary CSS values are constructed from user input.

'use client';

import { useSearchParams } from 'next/navigation';

const COLOR_MAP: Record<string, string> = {
  blue: 'text-blue-600',
  red: 'text-red-600',
  green: 'text-green-600',
  purple: 'text-purple-600',
  orange: 'text-orange-600',
} as const;

type AccentColor = keyof typeof COLOR_MAP;

function isValidColor(value: string): value is AccentColor {
  return value in COLOR_MAP;
}

export function ThemeCard({ children }: { children: React.ReactNode }) {
  const searchParams = useSearchParams();
  const rawColor = searchParams.get('accent') ?? 'blue';
  const accentColor = isValidColor(rawColor) ? rawColor : 'blue';
  const colorClass = COLOR_MAP[accentColor];

  return (
    <div className="p-4 border rounded shadow">
      <div className={`${colorClass} font-bold`}>
        User Profile
      </div>
      {children}
    </div>
  );
}
