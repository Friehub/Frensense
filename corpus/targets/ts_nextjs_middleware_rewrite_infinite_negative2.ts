// SAFE: enforces a maximum number of rewrites via a cookie counter

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const MAX_REWRITES = 3
  const rewriteCount = parseInt(request.cookies.get('_rewrite_count')?.value ?? '0', 10)
  if (rewriteCount >= MAX_REWRITES) {
    return NextResponse.next()
  }
  const country = request.cookies.get('country')?.value ?? 'us'
  const url = request.nextUrl.clone()
  url.pathname = `/${country}${url.pathname}`
  const response = NextResponse.rewrite(url)
  response.cookies.set('_rewrite_count', String(rewriteCount + 1))
  return response
}
