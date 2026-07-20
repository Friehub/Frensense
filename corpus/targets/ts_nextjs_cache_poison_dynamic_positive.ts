// [frensense]
// observation: The `use cache` directive is applied to a component that renders user-specific dashboard data without accounting for the current user identity in the cache key.
// impact: User A may receive User B's cached dashboard data, leaking personal information such as account balances and transaction history.
// improvement: Include the user ID or session identifier in the cache key when using the `use cache` directive on data that varies per user.

'use cache'

import { sql } from '@vercel/postgres'

export default async function Dashboard({ userId }: { userId: string }) {
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
