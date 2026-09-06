// SAFE: authentication checked via event.locals.user before processing

import type { RequestEvent } from '@sveltejs/kit'
import { db } from '$lib/database'

export async function GET(event: RequestEvent) {
  const session = event.locals.user
  if (!session) {
    return new Response('Unauthorized', { status: 401 })
  }

  const userId = event.url.searchParams.get('userId')
  if (Number(userId) !== session.id && session.role !== 'admin') {
    return new Response('Forbidden', { status: 403 })
  }

  const user = await db.user.findUnique({ where: { id: Number(userId) } })
  return new Response(JSON.stringify(user), { status: 200 })
}

export async function POST(event: RequestEvent) {
  const session = event.locals.user
  if (!session) {
    return new Response('Unauthorized', { status: 401 })
  }

  const body = await event.request.json()
  const updated = await db.user.update({ where: { id: body.id }, data: body })
  return new Response(JSON.stringify(updated), { status: 200 })
}
