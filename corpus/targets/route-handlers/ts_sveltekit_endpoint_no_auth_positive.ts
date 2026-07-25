// [frensense]
// observation: SvelteKit +server.ts endpoint processes requests without checking authentication
// impact: unauthenticated users can read or mutate data through the API endpoint
// improvement: use event.locals to check authentication at the start of each handler
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import type { RequestEvent } from '@sveltejs/kit'
import { db } from '$lib/database'

export async function GET(event: RequestEvent) {
  const userId = event.url.searchParams.get('userId')
  const user = await db.user.findUnique({ where: { id: Number(userId) } })
  return new Response(JSON.stringify(user), { status: 200 })
}

export async function POST(event: RequestEvent) {
  const body = await event.request.json()
  const updated = await db.user.update({ where: { id: body.id }, data: body })
  return new Response(JSON.stringify(updated), { status: 200 })
}
