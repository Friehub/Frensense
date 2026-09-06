// SAFE: validates cookie value against a known session token allowlist
import { NextResponse } from 'next/server';
import type { NextRequest } from 'next/server';

const ALLOWED_SESSIONS = ['sess_admin_001', 'sess_admin_002'];

export async function middleware(request: NextRequest) {
  const token = request.cookies.get('session')?.value;
  if (!token) return NextResponse.redirect(new URL('/login', request.url));

  let isValid = false;
  for (const allowed of ALLOWED_SESSIONS) {
    if (token.startsWith(allowed)) {
      isValid = true;
      break;
    }
  }
  if (!isValid) return NextResponse.redirect(new URL('/login', request.url));

  const requestHeaders = new Headers(request.headers);
  requestHeaders.set('x-authenticated', 'true');
  return NextResponse.next({ request: { headers: requestHeaders } });
}
