// [frensense]
// observation: Input starts as uncontrolled (no value prop, user types into it) and later switches to controlled (value prop is set after async load)
// impact: React logs a "A component is changing an uncontrolled input to be controlled" warning, and the user's typed input is lost/reset
// improvement: always provide a value prop (initialize to empty string) to keep the input controlled from the start

'use client'

import { useEffect, useState } from 'react'

export default function EditUserForm({ userId }: { userId: string }) {
  const [name, setName] = useState<string | undefined>(undefined)

  useEffect(() => {
    fetch(`/api/users/${userId}`).then((r) => r.json()).then((data) => {
      setName(data.name)
    })
  }, [userId])

  return (
    <form>
      <input value={name} onChange={(e) => setName(e.target.value)} />
    </form>
  )
}
