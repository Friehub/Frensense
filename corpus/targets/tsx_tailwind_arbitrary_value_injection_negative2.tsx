// SAFE: User input is sanitized to only allow valid hex color characters. The arbitrary value is constructed from the sanitized input, preventing CSS property injection through the bracket syntax.

'use client';

import { useSearchParams } from 'next/navigation';

const HEX_RE = /^#[0-9a-fA-F]{3,8}$/;

function sanitizeHex(input: string): string | null {
  const cleaned = `#${input.replace(/^#/, '').replace(/[^0-9a-fA-F]/g, '')}`;
  return HEX_RE.test(cleaned) ? cleaned : null;
}

export function ThemeCard({ children }: { children: React.ReactNode }) {
  const searchParams = useSearchParams();
  const rawHex = searchParams.get('accent') ?? '#3b82f6';
  const safeHex = sanitizeHex(rawHex) ?? '#3b82f6';

  return (
    <div className="p-4 border rounded shadow">
      <div className={`text-[color:${safeHex}] font-bold`}>
        User Profile
      </div>
      {children}
    </div>
  );
}
