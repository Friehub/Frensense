// [frensense]
// observation: Vite import.meta.env.VITE_SECRET_KEY referenced in client component bundle, exposing secrets to the browser.
// impact: Any VITE_-prefixed env var is inlined at build time into the client JS bundle, visible to all users via DevTools.
// improvement: Only use VITE_ prefix for values safe to expose. Use server-side env vars (process.env) for secrets, exposed via API routes.
// cwe: CWE-526
// cvss: 5.3
// owasp: A02:2021
// severity: Medium

export function ApiKeyDisplay() {
  return (
    <div>
      Current API key: {import.meta.env.VITE_SECRET_KEY}
    </div>
  );
}
