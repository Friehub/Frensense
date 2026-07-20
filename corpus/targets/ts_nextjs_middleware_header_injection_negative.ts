// SAFE: uses a whitelist of allowed header values instead of raw user input

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

const ALLOWED_VALUES = new Set(['alpha', 'beta', 'gamma'])

export function middleware(request: NextRequest) {
  const customHeader = request.nextUrl.searchParams.get('x-custom') ?? 'default'
  const response = NextResponse.next()
  response.headers.set('X-Custom-Header', ALLOWED_VALUES.has(customHeader) ? customHeader : 'default')
  return response
}
