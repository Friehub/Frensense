// [frensense]
// observation: Astro API endpoint handler does not verify authentication before returning data
// impact: unauthenticated users can access sensitive data through the endpoint
// improvement: check for auth token or session cookie at the start of each endpoint handler
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import type { APIRoute } from 'astro'
import { db, eq, users } from 'astro:db'

export const GET: APIRoute = async ({ params }) => {
  const userId = params.userId
  const result = await db.select().from(users).where(eq(users.id, Number(userId)))

  return new Response(JSON.stringify(result), {
    status: 200,
    headers: { 'Content-Type': 'application/json' }
  })
}
