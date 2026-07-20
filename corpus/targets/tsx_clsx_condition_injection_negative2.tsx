// SAFE: User input is passed as a key in a clsx conditions object, not as a raw string argument. Only truthy keys result in classes being applied, and prototype pollution via `__proto__` is prevented because clsx only accesses own enumerable properties of the argument object.

'use client';

import { useSearchParams } from 'next/navigation';
import clsx from 'clsx';

const SEVERITY_CLASSES = {
  info: 'bg-blue-50 border-blue-200 text-blue-800',
  warning: 'bg-yellow-50 border-yellow-200 text-yellow-800',
  error: 'bg-red-50 border-red-200 text-red-800',
  success: 'bg-green-50 border-green-200 text-green-800',
} as const;

export function AlertBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const rawSeverity = searchParams.get('severity') ?? 'info';

  return (
    <div
      className={clsx(
        'p-4 border rounded text-gray-800',
        SEVERITY_CLASSES[rawSeverity as keyof typeof SEVERITY_CLASSES] ??
          SEVERITY_CLASSES.info,
      )}
    >
      <p>{message}</p>
    </div>
  );
}
