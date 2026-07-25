// [frensense]
// observation: `useTransition` `isPending` flag is used to display the internal operation name in the loading indicator
// impact: attackers can observe which internal operation is running, leaking business logic and operation identifiers
// improvement: use a generic loading message that does not expose internal operation names
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use client'

import { useState, useTransition } from 'react'

export default function AdminPanel() {
  const [isPending, startTransition] = useTransition()
  const [result, setResult] = useState<string | null>(null)

  function handleDeleteUser(userId: string) {
    startTransition(async () => {
      const res = await fetch(`/api/admin/users/${userId}/delete`, { method: 'DELETE' })
      const data = await res.json()
      setResult(data.message)
    })
  }

  return (
    <div>
      <button onClick={() => handleDeleteUser('u-42')} disabled={isPending}>
        {isPending ? 'Running deleteUser(u-42) operation...' : 'Delete User'}
      </button>
      {result && <p>{result}</p>}
    </div>
  )
}
