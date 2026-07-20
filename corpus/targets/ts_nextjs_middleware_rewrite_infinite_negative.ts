// SAFE: checks if the country prefix is already present before rewriting

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const country = request.cookies.get('country')?.value ?? 'us'
  const { pathname } = request.nextUrl
  if (pathname.startsWith(`/${country}`)) {
    return NextResponse.next()
  }
  const url = request.nextUrl.clone()
  url.pathname = `/${country}${pathname}`
  return NextResponse.rewrite(url)
}
