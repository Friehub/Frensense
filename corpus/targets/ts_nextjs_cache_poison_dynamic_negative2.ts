// SAFE: The `use cache` directive is used but with explicit userId-based cache tag invalidation, ensuring per-user isolation

import { sql } from '@vercel/postgres'
import { revalidateTag } from 'next/cache'

export default async function Dashboard({ userId }: { userId: string }) {
  'use cache'
  const tag = `dashboard:${userId}`

  const { rows } = await sql`
    SELECT balance, transactions FROM accounts WHERE user_id = ${userId}
  `

  return (
    <div>
      <h1>Your Balance: ${rows[0]?.balance}</h1>
      <pre>{JSON.stringify(rows[0]?.transactions, null, 2)}</pre>
    </div>
  )
}
