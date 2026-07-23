// SAFE: Bearer token verified before returning data; only non-sensitive fields returned

import type { APIRoute } from 'astro'
import { db, eq, users } from 'astro:db'

async function verifyToken(authHeader: string | null): Promise<number | null> {
  if (!authHeader || !authHeader.startsWith('Bearer ')) return null
  const token = authHeader.slice(7)
  const session = await db.select().from(users).where(eq(users.apiToken, token)).get()
  return session?.id ?? null
}

export const GET: APIRoute = async ({ params, request }) => {
  const userId = await verifyToken(request.headers.get('Authorization'))
  if (!userId) {
    return new Response('Unauthorized', { status: 401 })
  }

  const result = await db.select({ id: users.id, name: users.name })
    .from(users)
    .where(eq(users.id, Number(params.userId)))

  return new Response(JSON.stringify(result), {
    status: 200,
    headers: { 'Content-Type': 'application/json' }
  })
}
