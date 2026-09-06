// SAFE: User input is validated against a predefined severity allowlist before being passed to clsx. Unrecognized values fall back to a default.

'use client';

import { useSearchParams } from 'next/navigation';
import clsx from 'clsx';

const SEVERITY_MAP: Record<string, string> = {
  info: 'bg-blue-50 border-blue-200 text-blue-800',
  warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
  error: 'bg-red-50 border-red-200 text-red-800',
  success: 'bg-green-50 border-green-200 text-green-800',
} as const;

type Severity = keyof typeof SEVERITY_MAP;

function isValidSeverity(value: string): value is Severity {
  return value in SEVERITY_MAP;
}

export function AlertBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const rawSeverity = searchParams.get('severity') ?? 'info';
  const severity = isValidSeverity(rawSeverity) ? rawSeverity : 'info';
  const severityClasses = SEVERITY_MAP[severity];

  return (
    <div className={clsx('p-4 border rounded', severityClasses, 'text-gray-800')}>
      <p>{message}</p>
    </div>
  );
}
