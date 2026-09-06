// [frensense]
// observation: User-controlled input is passed directly as an argument to `clsx()`. Since `clsx` accepts objects, arrays, and strings, an attacker can pass crafted input (e.g., `__proto__`, `constructor`) that triggers prototype pollution vulnerabilities or injects arbitrary class names that override existing CSS styles.
// impact: An attacker can inject class names that alter the visual appearance of the component (e.g., `hidden`, `pointer-events-none`, `opacity-0`) to hide or obfuscate UI elements. If the application uses isObject-like checks that fall through to prototype pollution, the attacker can pollute `Object.prototype` with malicious keys, leading to broader security impacts.
// improvement: Never pass user input directly to `clsx`. Validate user input against a predefined allowlist of class names, or use a mapping function that transforms user input into safe, known class names.

'use client';

import { useSearchParams } from 'next/navigation';
import clsx from 'clsx';

export function AlertBanner({ message }: { message: string }) {
  const searchParams = useSearchParams();
  const severity = searchParams.get('severity') ?? '';

  return (
    <div
      className={clsx(
        'p-4 border rounded',
        severity,
        'text-gray-800',
      )}
    >
      <p>{message}</p>
    </div>
  );
}
