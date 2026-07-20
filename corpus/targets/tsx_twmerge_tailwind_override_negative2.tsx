// SAFE: Security-critical classes are applied outside of `twMerge` using a separate style approach, ensuring they cannot be accidentally overridden by conflicting utility classes.

'use client';

import { twMerge } from 'tailwind-merge';

const VARIANT_STYLES = {
  danger: { backgroundColor: '#dc2626', color: '#ffffff' },
  success: { backgroundColor: '#16a34a', color: '#ffffff' },
} as const;

export function SecurityBadge({ variant }: { variant: 'success' | 'danger' }) {
  const baseClasses = 'px-3 py-1 rounded text-sm font-medium';

  return (
    <span
      className={twMerge(baseClasses)}
      style={VARIANT_STYLES[variant]}
    >
      {variant === 'danger' ? 'Suspicious Activity' : 'Verified Secure'}
    </span>
  );
}
