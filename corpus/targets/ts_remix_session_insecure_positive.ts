// [frensense]
// observation: Remix session storage is configured with an empty or weak secret, allowing an attacker to forge or tamper session cookies
// impact: An attacker can craft arbitrary session cookies, impersonate any user, or tamper with session data
// improvement: Use a strong random secret from an environment variable (e.g., process.env.SESSION_SECRET) with at least 32 characters
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

import { createCookieSessionStorage } from '@remix-run/node';

function createSessionStorage() {
  return createCookieSessionStorage({
    cookie: {
      name: '__session',
      secrets: [''],
      sameSite: 'lax',
    },
  });
}
