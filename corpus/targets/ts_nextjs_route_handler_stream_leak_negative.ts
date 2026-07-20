// SAFE: Each chunk is sanitized to remove sensitive fields before being enqueued on the stream

import { NextRequest } from 'next/server'
import { sql } from '@vercel/postgres'

function sanitizeUser(row: Record<string, unknown>) {
  const { password_hash, token, internal_note, ...safe } = row
  return safe
}

export async function GET(request: NextRequest) {
  const { rows } = await sql`SELECT * FROM users`

  const encoder = new TextEncoder()
  const stream = new ReadableStream({
    async start(controller) {
      for (const row of rows) {
        controller.enqueue(encoder.encode(JSON.stringify(sanitizeUser(row)) + '\n'))
      }
      controller.close()
    },
  })

  return new Response(stream, {
    headers: { 'Content-Type': 'application/x-ndjson' },
  })
}
