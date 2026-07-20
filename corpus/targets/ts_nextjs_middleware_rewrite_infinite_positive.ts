// [frensense]
// observation: middleware.ts rewrites based on user input without termination check, causing infinite redirect loop
// impact: browser exhausts redirect limit, user stuck on error page, denial of service
// improvement: add a redirect counter cookie or pathname check to break the loop

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const country = request.cookies.get('country')?.value ?? 'us'
  const url = request.nextUrl.clone()
  url.pathname = `/${country}${url.pathname}`
  return NextResponse.rewrite(url)
}
