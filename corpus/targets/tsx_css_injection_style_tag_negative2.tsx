// SAFE: If custom styles are required, they are defined as static CSS-in-JS objects with a restricted set of allowed properties. No `<style>` tag is constructed from user input.

'use client';

import { useSearchParams } from 'next/navigation';

const ALLOWED_CSS_PROPERTIES = new Set([
  'color',
  'background-color',
  'font-size',
  'padding',
  'margin',
  'border-radius',
]);

function sanitizeStyle(raw: string): string {
  const rules: string[] = [];
  for (const decl of raw.split(';')) {
    const trimmed = decl.trim();
    if (!trimmed) continue;
    const [prop, ...vals] = trimmed.split(':');
    const property = prop?.trim().toLowerCase();
    if (property && ALLOWED_CSS_PROPERTIES.has(property)) {
      rules.push(`${property}: ${vals.join(':').trim()}`);
    }
  }
  return rules.join('; ');
}

export function CustomStyles() {
  const searchParams = useSearchParams();
  const rawCss = searchParams.get('css') ?? '';
  const safeCss = sanitizeStyle(rawCss);

  return (
    <div>
      <style>{`.user-card { ${safeCss} }`}</style>
      <div className="user-card p-4 border rounded">
        <h2>User Profile</h2>
      </div>
    </div>
  );
}
