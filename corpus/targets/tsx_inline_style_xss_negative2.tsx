// SAFE: Both CSS property names and values are drawn from a predefined allowlist. No user input is used directly as a CSS property or value.

'use client';

import { useSearchParams } from 'next/navigation';

const ALLOWED_STYLES: Record<string, Record<string, string>> = {
  primary: { backgroundColor: '#3b82f6', color: '#ffffff' },
  secondary: { backgroundColor: '#6b7280', color: '#ffffff' },
  danger: { backgroundColor: '#ef4444', color: '#ffffff' },
  success: { backgroundColor: '#22c55e', color: '#ffffff' },
} as const;

type StyleVariant = keyof typeof ALLOWED_STYLES;

function isValidVariant(value: string): value is StyleVariant {
  return value in ALLOWED_STYLES;
}

export function DynamicStyleBox() {
  const searchParams = useSearchParams();
  const rawVariant = searchParams.get('variant') ?? 'primary';
  const variant = isValidVariant(rawVariant) ? rawVariant : 'primary';
  const inlineStyles = ALLOWED_STYLES[variant];

  return (
    <div className="p-4 border rounded" style={inlineStyles}>
      <h2>Custom Styled Box</h2>
    </div>
  );
}
