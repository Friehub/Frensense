// SAFE: User input is only used for the CSS value, not the property name. CSS property values are sanitized to block dangerous URL schemes (`javascript:`, `data:`, `vbscript:`).

'use client';

import { useSearchParams } from 'next/navigation';

const DANGEROUS_SCHEMES = ['javascript:', 'data:', 'vbscript:', 'expression('];

function isSafeCssValue(value: string): boolean {
  return !DANGEROUS_SCHEMES.some((scheme) =>
    value.toLowerCase().includes(scheme),
  );
}

export function DynamicStyleBox() {
  const searchParams = useSearchParams();
  const cssValue = searchParams.get('value') ?? 'red';
  const safeValue = isSafeCssValue(cssValue) ? cssValue : 'red';

  return (
    <div
      className="p-4 border rounded"
      style={{ backgroundColor: safeValue }}
    >
      <h2>Custom Styled Box</h2>
    </div>
  );
}
