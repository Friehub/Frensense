// SAFE: Middleware reads a signed session cookie and verifies the HMAC signature using a server secret before trusting its value

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'
import { createHmac, timingSafeEqual } from 'node:crypto'

const SESSION_SECRET = process.env.SESSION_SECRET ?? ''

function verifySignedCookie(value: string): string | null {
  const idx = value.lastIndexOf('.')
  if (idx === -1) return null
  const payload = value.slice(0, idx)
  const sig = value.slice(idx + 1)
  const expected = createHmac('sha256', SESSION_SECRET).update(payload).digest('hex')
  if (sig.length !== expected.length) return null
  try {
    return timingSafeEqual(Buffer.from(sig), Buffer.from(expected)) ? payload : null
  } catch {
    return null
  }
}

export function middleware(request: NextRequest) {
  const raw = request.cookies.get('session_id')?.value
  if (!raw) return NextResponse.redirect(new URL('/login', request.url))

  const sessionId = verifySignedCookie(raw)
  if (!sessionId) return NextResponse.redirect(new URL('/login', request.url))

  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('x-session-id', sessionId)
  return NextResponse.next({ request: { headers: requestHeaders } })
}

export const config = {
  matcher: '/dashboard/:path*',
}
