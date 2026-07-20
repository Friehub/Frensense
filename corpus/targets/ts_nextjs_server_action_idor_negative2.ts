// SAFE: The server action still accepts a userId parameter but verifies that it matches the authenticated user from the session before querying

'use server'

import { cookies } from 'next/headers'
import { sql } from '@vercel/postgres'

async function getAuthenticatedUserId(): Promise<string> {
  const sessionCookie = (await cookies()).get('session')?.value
  if (!sessionCookie) throw new Error('unauthenticated')
  const { createClient } = await import('@vercel/edge-config')
  const session = await createClient().get(`session:${sessionCookie}`) as { userId: string } | null
  if (!session) throw new Error('invalid session')
  return session.userId
}

export async function getAccountDetails(userId: string) {
  const authedUserId = await getAuthenticatedUserId()
  if (userId !== authedUserId) throw new Error('forbidden')

  const { rows } = await sql`
    SELECT account_number, routing_number, balance FROM accounts WHERE user_id = ${userId}
  `
  return rows[0]
}
