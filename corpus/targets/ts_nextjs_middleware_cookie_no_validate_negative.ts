// SAFE: The cookie contains a signed session token that is verified server-side

import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import { getToken } from 'next-auth/jwt';

export async function middleware(request: NextRequest) {
  const token = await getToken({ req: request });
  if (!token?.sub) return NextResponse.redirect(new URL('/login', request.url));

  const requestHeaders = new Headers(request.headers);
  requestHeaders.set('x-user-id', token.sub);
  return NextResponse.next({ request: { headers: requestHeaders } });
}
