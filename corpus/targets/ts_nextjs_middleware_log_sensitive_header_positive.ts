// [frensense]
// observation: Middleware logs all request headers for debugging purposes, including sensitive headers like `authorization`, `cookie`, and `x-api-key`, to stdout which may be collected by log aggregation services.
// impact: Session tokens, API keys, and authentication credentials from every request are written to logs, allowing anyone with log access to steal active sessions (CVE-2025-47764 variant).
// improvement: Strip sensitive headers before logging, or use a structured logger with redaction support.

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

export function middleware(request: NextRequest) {
  const headers: Record<string, string> = {}
  request.headers.forEach((value, key) => {
    headers[key] = value
  })
  console.log(`[middleware] ${request.method} ${request.url} headers=${JSON.stringify(headers)}`)
  return NextResponse.next()
}
