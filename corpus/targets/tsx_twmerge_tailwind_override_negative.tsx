// SAFE: The default classes and variant classes are passed after the conflicting override class, so `twMerge` preserves the intended variant styles (later classes win).

'use client';

import { twMerge } from 'tailwind-merge';

export function SecurityBadge({ variant }: { variant: 'success' | 'danger' }) {
  const baseClasses = 'px-3 py-1 rounded text-sm font-medium';
  const variantClasses = variant === 'danger'
    ? 'bg-red-600 text-white'
    : 'bg-green-600 text-white';

  return (
    <span className={twMerge(baseClasses, variantClasses)}>
      {variant === 'danger' ? 'Suspicious Activity' : 'Verified Secure'}
    </span>
  );
}
