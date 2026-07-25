// [frensense]
// observation: A streaming route handler returns raw database rows or internal objects chunk-by-chunk without filtering sensitive fields like passwords, tokens, or internal IDs.
// impact: Sensitive data fields are streamed to the client as individual chunks, potentially exposing credentials or internal identifiers.
// improvement: Apply a map or filter transform on each chunk before writing it to the response stream.
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

import { NextRequest } from 'next/server'
import { sql } from '@vercel/postgres'

export async function GET(request: NextRequest) {
  const { rows } = await sql`SELECT * FROM users`

  const encoder = new TextEncoder()
  const stream = new ReadableStream({
    async start(controller) {
      for (const row of rows) {
        controller.enqueue(encoder.encode(JSON.stringify(row) + '\n'))
      }
      controller.close()
    },
  })

  return new Response(stream, {
    headers: { 'Content-Type': 'application/x-ndjson' },
  })
}
