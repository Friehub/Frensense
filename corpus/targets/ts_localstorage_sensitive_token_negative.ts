// SAFE: Store auth tokens in an httpOnly, Secure, SameSite cookie instead of localStorage.

import { serialize } from 'node:cookie';

function setAuthCookie(token: string): void {
  const cookie = serialize('authToken', token, {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    path: '/api',
    maxAge: 900,
  });
  document.cookie = cookie;
}

async function fetchWithAuth(url: string): Promise<Response> {
  return fetch(url, {
    credentials: 'include',
  });
}
