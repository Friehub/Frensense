// SAFE: User-controlled classes are sanitized to strip `!important` prefixes before being applied, preventing attackers from overriding security-critical styles.

'use client';

import { useSearchParams } from 'next/navigation';

function stripImportant(classes: string): string {
  return classes
    .split(/\s+/)
    .filter(Boolean)
    .filter((c) => !c.startsWith('!'))
    .join(' ');
}

export function SecurityBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const extra = searchParams.get('class') ?? '';
  const safeClasses = stripImportant(extra);

  return (
    <div className="relative">
      <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
        <div className={`bg-white p-6 rounded-lg shadow-xl ${safeClasses}`}>
          <h2 className="text-xl font-bold text-red-600">Security Warning</h2>
          <p>{message}</p>
        </div>
      </div>
    </div>
  );
}
