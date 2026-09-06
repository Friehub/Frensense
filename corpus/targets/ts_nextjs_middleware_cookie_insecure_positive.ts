// [frensense]
// observation: Middleware reads an unsigned `session_id` cookie and uses it for authentication decisions without verifying a cryptographic signature or MAC.
// impact: An attacker can forge a `session_id` cookie value to impersonate any user, bypassing all authentication in middleware.
// improvement: Use a signed or encrypted session cookie, or validate the session token against a server-side store.
// cwe: CWE-614
// cvss: 5.4
// owasp: A02:2021
// severity: Medium

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const sessionId = request.cookies.get('session_id')?.value
  if (!sessionId) {
    return NextResponse.redirect(new URL('/login', request.url))
  }

  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('x-session-id', sessionId)
  return NextResponse.next({ request: { headers: requestHeaders } })
}

export const config = {
  matcher: '/dashboard/:path*',
}
