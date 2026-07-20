// SAFE: Sensitive headers are redacted before logging, preventing credential exposure in logs

import { NextResponse } from 'next/server'
import type { NextRequest } from 'next/server'

const SENSITIVE_HEADERS = new Set(['authorization', 'cookie', 'set-cookie', 'x-api-key', 'x-session-token'])

function redactHeaders(headers: Headers): Record<string, string> {
  const safe: Record<string, string> = {}
  headers.forEach((value, key) => {
    safe[key] = SENSITIVE_HEADERS.has(key.toLowerCase()) ? '[REDACTED]' : value
  })
  return safe
}

export function middleware(request: NextRequest) {
  const safe = redactHeaders(request.headers)
  console.log(`[middleware] ${request.method} ${request.url} headers=${JSON.stringify(safe)}`)
  return NextResponse.next()
}
