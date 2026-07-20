// SAFE: sanitizes header value by removing CRLF characters

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

function sanitizeHeaderValue(value: string): string {
  return value.replace(/[\r\n]/g, '').trim()
}

export function middleware(request: NextRequest) {
  const customHeader = request.nextUrl.searchParams.get('x-custom') ?? 'default'
  const response = NextResponse.next()
  response.headers.set('X-Custom-Header', sanitizeHeaderValue(customHeader))
  return response
}
