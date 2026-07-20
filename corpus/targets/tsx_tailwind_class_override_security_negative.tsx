// SAFE: User input is not applied to security-critical overlay elements. Non-critical decorative classes are mapped through an allowlist and never include `!important` variants.

'use client';

import { useSearchParams } from 'next/navigation';

const ALLOWED_CARD_STYLES: Record<string, string> = {
  default: 'max-w-md',
  wide: 'max-w-lg',
  narrow: 'max-w-sm',
} as const;

type CardStyle = keyof typeof ALLOWED_CARD_STYLES;

function isValidStyle(value: string): value is CardStyle {
  return value in ALLOWED_CARD_STYLES;
}

export function SecurityBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const rawStyle = searchParams.get('style') ?? 'default';
  const cardStyle = isValidStyle(rawStyle) ? rawStyle : 'default';
  const styleClass = ALLOWED_CARD_STYLES[cardStyle];

  return (
    <div className="relative">
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div className={`bg-white p-6 rounded-lg shadow-xl ${styleClass}`}>
          <h2 className="text-xl font-bold text-red-600">Security Warning</h2>
          <p>{message}</p>
        </div>
      </div>
    </div>
  );
}
