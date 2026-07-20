// [frensense]
// observation: Next.js 16.3 middleware runs in the Node.js runtime (stable) and uses `fetch()` with a URL derived from request headers or query parameters, making SSRF attacks possible against internal services.
// impact: An attacker can inject a URL like `http://169.254.169.254/latest/meta-data/` to exfiltrate cloud metadata from the middleware runtime, or probe internal network services (SSRF).
// improvement: Validate and restrict fetch URLs to an allowlist of known external endpoints in middleware; never use user input directly in the fetch URL.

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export async function middleware(request: NextRequest) {
  const proxyUrl = request.nextUrl.searchParams.get('proxy')
  if (proxyUrl) {
    const res = await fetch(proxyUrl)
    const data = await res.text()
    return new NextResponse(data)
  }
  return NextResponse.next()
}

export const config = {
  matcher: '/api/proxy',
}
