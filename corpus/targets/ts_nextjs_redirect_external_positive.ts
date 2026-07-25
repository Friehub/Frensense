// [frensense]
// observation: Next.js redirect() is called with a user-controlled URL, enabling open redirect attacks.
// impact: An attacker can trick users into visiting malicious external sites via a redirect from a trusted domain.
// improvement: Validate that the redirect URL is relative or on an allowlist before calling redirect().
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// severity: Medium
// runtime_probe: redirect

import { redirect } from 'next/navigation';

export async function GET(request: Request) {
  const url = new URL(request.url);
  const dest = url.searchParams.get('redirect');
  redirect(dest || '/');
}
