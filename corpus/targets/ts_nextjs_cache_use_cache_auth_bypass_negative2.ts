// SAFE: User-specific data is fetched via a dynamic server component without caching, ensuring per-user isolation

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
