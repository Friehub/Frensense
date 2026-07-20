// SAFE: The URL used in fetch is validated against an allowlist of permitted origins

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

const ALLOWED_PROXY_ORIGINS = ['https://api.example.com', 'https://data.example.com']

function isAllowed(url: string): boolean {
  try {
    const parsed = new URL(url)
    return ALLOWED_PROXY_ORIGINS.some((origin) => parsed.origin === origin)
  } catch {
    return false
  }
}

export async function middleware(request: NextRequest) {
  const proxyUrl = request.nextUrl.searchParams.get('proxy')
  if (proxyUrl && isAllowed(proxyUrl)) {
    const res = await fetch(proxyUrl)
    const data = await res.text()
    return new NextResponse(data)
  }
  return NextResponse.next()
}

export const config = {
  matcher: '/api/proxy',
}
