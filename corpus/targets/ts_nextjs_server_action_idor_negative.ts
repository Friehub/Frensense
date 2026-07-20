// SAFE: The authenticated user ID is read from the session, and the client-supplied userId parameter is ignored for authorization

'use server'

import { cookies } from 'next/headers'
import { sql } from '@vercel/postgres'
import { createClient } from '@vercel/edge-config'

export async function getAccountDetails(_ignoredUserId: string) {
  const sessionCookie = (await cookies()).get('session')?.value
  if (!sessionCookie) throw new Error('unauthenticated')

  const session = await createClient().get(`session:${sessionCookie}`) as { userId: string } | null
  if (!session) throw new Error('invalid session')

  const { rows } = await sql`
    SELECT account_number, routing_number, balance FROM accounts WHERE user_id = ${session.userId}
  `
  return rows[0]
}
