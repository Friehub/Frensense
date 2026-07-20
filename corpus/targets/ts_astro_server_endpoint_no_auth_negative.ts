// SAFE: authentication verified via cookie or auth header before data access

import type { APIRoute } from 'astro'
import { db, eq, users } from 'astro:db'

export const GET: APIRoute = async ({ params, cookies }) => {
  const sessionToken = cookies.get('session_token')?.value
  if (!sessionToken) {
    return new Response('Unauthorized', { status: 401 })
  }

  const session = await db.select().from(users).where(eq(users.sessionToken, sessionToken)).get()
  if (!session) {
    return new Response('Unauthorized', { status: 401 })
  }

  const userId = params.userId
  const result = await db.select().from(users).where(eq(users.id, Number(userId)))

  return new Response(JSON.stringify(result), {
    status: 200,
    headers: { 'Content-Type': 'application/json' }
  })
}
