// SAFE: User input for animation speed is validated and clamped to a safe range (0.5–5 seconds). Only the duration parameter is allowed, preventing injection of additional animation properties.

'use client';

import { useSearchParams } from 'next/navigation';

const MIN_DURATION = 0.5;
const MAX_DURATION = 5;

function clampDuration(raw: string): number {
  const parsed = Number.parseFloat(raw);
  if (Number.isNaN(parsed)) return 1;
  return Math.min(MAX_DURATION, Math.max(MIN_DURATION, parsed));
}

export function LoadingSpinner() {
  const searchParams = useSearchParams();
  const rawSpeed = searchParams.get('speed') ?? '1';
  const speed = clampDuration(rawSpeed);

  return (
    <div className="flex items-center justify-center p-8">
      <div
        className={`w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-[spin_${speed}s_linear_infinite]`}
      />
    </div>
  );
}
