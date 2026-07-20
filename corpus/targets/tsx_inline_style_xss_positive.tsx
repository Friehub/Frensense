// [frensense]
// observation: A user-controlled object key (CSS property) is used in a React `style` prop. An attacker can provide dangerous CSS property names like `javascript:` URLs (e.g., `background-image: url(javascript:alert(1))`) or IE-specific `expression()` values (e.g., `background: expression(alert(1))`). In older browsers, CSS `expression()` can execute arbitrary JavaScript.
// impact: An attacker can achieve Cross-Site Scripting (XSS) via CSS injection by providing `javascript:` URLs in CSS properties like `background-image`, `cursor`, or `behavior`. While rare in modern browsers (Chrome/Firefox block javascript: in CSS URLs), older browsers (IE6-9) and some WebView implementations execute `expression()` and `javascript:` URLs, making this a threat for legacy browser support or Electron apps.
// improvement: Validate CSS property names and values against an allowlist. Never allow `javascript:`, `expression()`, or `data:` URLs in CSS property values. Avoid using user input directly as CSS property keys.

'use client';

import { useSearchParams } from 'next/navigation';

export function DynamicStyleBox() {
  const searchParams = useSearchParams();
  const cssProperty = searchParams.get('property') ?? 'background-color';
  const cssValue = searchParams.get('value') ?? 'red';

  return (
    <div
      className="p-4 border rounded"
      style={{ [cssProperty]: cssValue }}
    >
      <h2>Custom Styled Box</h2>
    </div>
  );
}
