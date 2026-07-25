// [frensense]
// observation: Partial Prerendering (PPR) streams the page shell synchronously while async content loads. When a cookie-parsing error occurs during streaming, the error message includes the raw cookie header value, which gets serialized into the streamed response.
// impact: An attacker can trigger a cookie parsing error (e.g., by sending a malformed cookie) and receive the raw cookie header in the streaming error response, leaking session tokens from other users if the error is cached (CVE-2025-47764 variant).
// improvement: Wrap cookie parsing in try/catch and return a generic error message; never include raw request header values in error responses.
// cwe: CWE-614
// cvss: 5.4
// owasp: A02:2021
// severity: Medium

import { cookies } from 'next/headers'

export default async function AppLayout({ children }: { children: React.ReactNode }) {
  const sessionCookie = (await cookies()).get('session')
  if (!sessionCookie) {
    throw new Error(`Missing session cookie. Raw cookie header: ${(await cookies()).toString()}`)
  }
  return <div>{children}</div>
}
