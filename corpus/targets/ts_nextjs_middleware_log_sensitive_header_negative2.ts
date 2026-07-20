// SAFE: Headers are not logged at all in middleware; structured logging is used externally

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  return NextResponse.next()
}
