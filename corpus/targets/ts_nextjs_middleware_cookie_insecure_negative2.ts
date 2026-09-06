// SAFE: Middleware does not trust the cookie value directly; it validates the session token against a Redis store

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'
import { createClient } from 'redis'

const redis = createClient({ url: process.env.REDIS_URL })

await redis.connect()

export async function middleware(request: NextRequest) {
  const token = request.cookies.get('session_token')?.value
  if (!token) return NextResponse.redirect(new URL('/login', request.url))

  const sessionData = await redis.get(`session:${token}`)
  if (!sessionData) return NextResponse.redirect(new URL('/login', request.url))

  const requestHeaders = new Headers(request.headers)
  requestHeaders.set('x-session-data', sessionData)
  return NextResponse.next({ request: { headers: requestHeaders } })
}

export const config = {
  matcher: '/dashboard/:path*',
}
