// [frensense]
// observation: Remix session storage is configured with an empty or weak secret, allowing an attacker to forge or tamper session cookies
// impact: An attacker can craft arbitrary session cookies, impersonate any user, or tamper with session data
// improvement: Use a strong random secret from an environment variable (e.g., process.env.SESSION_SECRET) with at least 32 characters

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
