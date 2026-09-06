// [frensense]
// observation: `'use client'` component imports a server-only module (database client, internal tokens) causing the server-only code to be bundled into the client JavaScript
// impact: server-side secrets, database credentials, and internal APIs are exposed in the client bundle
// improvement: keep server-only imports in server components; pass data as props from server to client
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { useEffect, useState } from 'react'
import db from '@/lib/database'
import { INTERNAL_API_KEY } from '@/lib/secrets'

export default function UserProfile({ userId }: { userId: string }) {
  const [user, setUser] = useState<{ name: string } | null>(null)

  useEffect(() => {
    db.query('SELECT * FROM users WHERE id = ?', [userId]).then((rows) => {
      setUser(rows[0] as { name: string })
    })
  }, [userId])

  if (!user) return <div>Loading...</div>
  return <div>{user.name}</div>
}
