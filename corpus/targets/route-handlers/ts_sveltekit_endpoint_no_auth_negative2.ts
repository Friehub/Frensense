// SAFE: handle hook in hooks.server.ts enforces auth, endpoints assume authenticated

import type { RequestEvent } from '@sveltejs/kit'
import { db } from '$lib/database'

export async function GET(event: RequestEvent) {
  const userId = event.url.searchParams.get('userId')
  if (Number(userId) !== event.locals.user.id && event.locals.user.role !== 'admin') {
    return new Response('Forbidden', { status: 403 })
  }

  const user = await db.user.findUnique({ where: { id: Number(userId) } })
  return new Response(JSON.stringify(user), { status: 200 })
}

export async function POST(event: RequestEvent) {
  const body = await event.request.json()
  const updated = await db.user.update({ where: { id: body.id }, data: body })
  return new Response(JSON.stringify(updated), { status: 200 })
}
