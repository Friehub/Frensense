// [frensense]
// observation: Next.js middleware reads a cookie value and uses it directly in authorization decisions without validating its authenticity or signature.
// impact: An attacker can forge or tamper with cookies to impersonate any user or bypass authorization.
// improvement: Verify the cookie signature or validate the session server-side before trusting cookie values.

import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  const userId = request.cookies.get('user_id')?.value;
  if (!userId) return NextResponse.redirect(new URL('/login', request.url));

  const requestHeaders = new Headers(request.headers);
  requestHeaders.set('x-user-id', userId);
  return NextResponse.next({ request: { headers: requestHeaders } });
}
