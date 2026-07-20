// SAFE: The cache entry is tagged with a user-specific tag, preventing cross-user cache hits

'use cache'

import { sql } from '@vercel/postgres'
import { unstable_cacheTag as cacheTag } from 'next/cache'

export default async function Notifications({ userId }: { userId: string }) {
  cacheTag(`notifications:${userId}`)

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
