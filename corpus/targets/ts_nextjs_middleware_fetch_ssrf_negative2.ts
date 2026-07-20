// SAFE: Proxy parameter is removed in middleware; all fetch URLs are hardcoded and not user-controllable

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export async function middleware(request: NextRequest) {
  const target = new URL(request.url)
  target.searchParams.delete('proxy')

  const res = await fetch('https://api.example.com/verify', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify({ path: target.pathname }),
  })

  if (!res.ok) {
    return new NextResponse('Access denied', { status: 403 })
  }

  return NextResponse.next()
}

export const config = {
  matcher: '/api/protected',
}
