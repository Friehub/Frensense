// SAFE: User-controlled z-index input is validated and clamped to a safe range (1–100), preventing the overlay from appearing above security chrome (typically z-index > 2^31 or very high values like 99999).

'use client';

import { useSearchParams } from 'next/navigation';

const MAX_Z_INDEX = 100;

function clampZIndex(raw: string): number {
  const parsed = Number.parseInt(raw, 10);
  if (Number.isNaN(parsed)) return 50;
  return Math.min(MAX_Z_INDEX, Math.max(1, parsed));
}

export function OverlayAd() {
  const searchParams = useSearchParams();
  const rawZ = searchParams.get('z') ?? '50';
  const zIndex = clampZIndex(rawZ);

  return (
    <div
      className="fixed inset-0 flex items-center justify-center bg-white/90"
      style={{ zIndex }}
    >
      <div className="p-6 bg-white rounded-lg shadow-2xl">
        <h2>Welcome!</h2>
        <button className="bg-blue-500 text-white px-6 py-2 rounded">
          Continue
        </button>
      </div>
    </div>
  );
}
