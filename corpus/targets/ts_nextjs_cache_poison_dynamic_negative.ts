// SAFE: `use cache` directive is not used for user-specific data; caching is handled explicitly with a unique per-user cache tag
// CVE: CVE-2025-55183 (variant)

import { sql } from '@vercel/postgres'
import { revalidateTag } from 'next/cache'
import { cache } from 'react'
import { unstable_noStore } from 'next/cache'

const getUserData = cache(async (userId: string) => {
  const { rows } = await sql`
    SELECT balance, transactions FROM accounts WHERE user_id = ${userId}
  `
  return rows[0]
})

export default async function Dashboard({ userId }: { userId: string }) {
  unstable_noStore()
  const data = await getUserData(userId)

  return (
    <div>
      <h1>Your Balance: ${data?.balance}</h1>
      <pre>{JSON.stringify(data?.transactions, null, 2)}</pre>
    </div>
  )
}
