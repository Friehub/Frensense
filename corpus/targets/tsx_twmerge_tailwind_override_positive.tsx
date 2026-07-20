// [frensense]
// observation: Tailwind utility classes are passed to `twMerge()` in an order where conflicting security-related classes can be unintentionally removed. `twMerge` resolves conflicting Tailwind classes by keeping only the last one in the merge order. If a security class like `bg-red-500` (warning background) is specified before a conflicting utility class like `bg-white`, the security class is silently dropped without warning.
// impact: An attacker or developer error can cause security-critical styling (e.g., error backgrounds, warning colors, focus outlines, disabled states) to be silently removed by `twMerge`. This can hide security warnings, disable visual indicators of dangerous actions, or remove focus-visible outlines needed for accessibility security. The class resolution is opaque and makes subtle UI security bugs hard to detect.
// improvement: Ensure security-critical classes are placed after conflicting classes in `twMerge()` arguments, or use a dedicated security class wrapper that is not passed through `twMerge`. Consider using a hardened version of twMerge that preserves certain critical classes.

'use client';

import { twMerge } from 'tailwind-merge';

export function SecurityBadge({ variant }: { variant: 'success' | 'danger' }) {
  const baseClasses = 'px-3 py-1 rounded text-white text-sm font-medium';
  const variantClasses = variant === 'danger'
    ? 'bg-red-600 text-white'
    : 'bg-green-600 text-white';

  return (
    <span className={twMerge(baseClasses, variantClasses, 'bg-white')}>
      {variant === 'danger' ? 'Suspicious Activity' : 'Verified Secure'}
    </span>
  );
}
