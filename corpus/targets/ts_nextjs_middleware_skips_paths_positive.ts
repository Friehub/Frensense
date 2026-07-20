// [frensense]
// observation: Next.js middleware uses a matcher that covers most paths but omits sensitive API or auth routes.
// impact: Attackers can directly call unprotected API routes or auth callback endpoints without middleware security checks.
// improvement: Ensure the matcher includes all paths that need protection, or use a deny-by-default approach.

import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

export function middleware(request: NextRequest) {
  const token = request.cookies.get('session-token');
  if (!token) return NextResponse.redirect(new URL('/login', request.url));
  return NextResponse.next();
}

export const config = {
  matcher: ['/((?!api|_next/static|_next/image|favicon.ico).*)'],
};
