// [frensense]
// observation: A server component uses the `'use cache'` directive to fetch and render user-specific data (e.g., account details, notifications) without tagging the cache entry with a user-specific identifier.
// impact: User A's private data is served from cache to User B because the cache key only includes the URL path, not the user identity, causing cross-user data exposure.
// improvement: Include a user-specific tag in the cache entry using the `tag` option or add the user ID to the cache key.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

'use cache'

import { sql } from '@vercel/postgres'

export default async function Notifications({ userId }: { userId: string }) {
  const { rows } = await sql`
    SELECT message, link FROM notifications WHERE user_id = ${userId} ORDER BY created_at DESC LIMIT 10
  `

  return (
    <ul>
      {rows.map((row) => (
        <li key={row.link}><a href={row.link}>{row.message}</a></li>
      ))}
    </ul>
  )
}
