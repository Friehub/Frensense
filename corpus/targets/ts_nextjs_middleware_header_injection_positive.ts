// [frensense]
// observation: middleware sets a response header directly from user-controlled query parameter
// impact: attacker injects arbitrary HTTP headers (CRLF, XSS, cache poisoning)
// improvement: validate or sanitize header values before setting them

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const customHeader = request.nextUrl.searchParams.get('x-custom') ?? 'default'
  const response = NextResponse.next()
  response.headers.set('X-Custom-Header', customHeader)
  return response
}
