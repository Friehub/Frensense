// SAFE: A whitelist-based projection is applied to each row before streaming, ensuring only explicitly allowed fields are sent

import { NextRequest } from 'next/server'
import { sql } from '@vercel/postgres'

const ALLOWED_FIELDS = new Set(['id', 'name', 'email', 'avatar_url'])

function pickSafe(row: Record<string, unknown>): Record<string, unknown> {
  const safe: Record<string, unknown> = {}
  for (const key of ALLOWED_FIELDS) {
    if (key in row) safe[key] = row[key]
  }
  return safe
}

export async function GET(request: NextRequest) {
  const { rows } = await sql`SELECT * FROM users`

  const encoder = new TextEncoder()
  const stream = new ReadableStream({
    async start(controller) {
      for (const row of rows) {
        controller.enqueue(encoder.encode(JSON.stringify(pickSafe(row)) + '\n'))
      }
      controller.close()
    },
  })

  return new Response(stream, {
    headers: { 'Content-Type': 'application/x-ndjson' },
  })
}
