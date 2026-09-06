// SAFE: Cookie value is a JWT verified with jsonwebtoken before use

import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';
import jwt from 'jsonwebtoken';

export async function middleware(request: NextRequest) {
  const sessionCookie = request.cookies.get('session')?.value;
  if (!sessionCookie) return NextResponse.redirect(new URL('/login', request.url));

  try {
    const payload = jwt.verify(sessionCookie, process.env.JWT_SECRET!) as { sub: string };
    const requestHeaders = new Headers(request.headers);
    requestHeaders.set('x-user-id', payload.sub);
    return NextResponse.next({ request: { headers: requestHeaders } });
  } catch {
    return NextResponse.redirect(new URL('/login', request.url));
  }
}
