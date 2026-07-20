// [frensense]
// observation: A Server Action accepts a `userId` parameter from the client and uses it to look up or modify data without verifying that the authenticated user owns that userId.
// impact: An attacker can supply any arbitrary `userId` to read or modify other users' data, leading to horizontal privilege escalation.
// improvement: Derive the user identity from the session token rather than trusting client-provided parameters, or verify ownership before acting.

'use server'

import { sql } from '@vercel/postgres'

export async function getAccountDetails(userId: string) {
  const { rows } = await sql`
    SELECT account_number, routing_number, balance FROM accounts WHERE user_id = ${userId}
  `
  return rows[0]
}
