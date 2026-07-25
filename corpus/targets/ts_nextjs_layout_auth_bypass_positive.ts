// [frensense]
// observation: the parent layout checks auth but the child `/admin/settings` page exists in a nested route segment that skips the layout guard
// impact: nested route bypasses the auth check, granting unauthenticated access to admin features
// improvement: move auth check to a middleware or enforce it per route group
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { ReactNode } from 'react'
import { redirect } from 'next/navigation'
import { getSession } from '@/lib/session'

export default async function AdminLayout({ children }: { children: ReactNode }) {
  const session = await getSession()
  if (!session) redirect('/login')
  return (
    <div>
      <nav>Admin Nav</nav>
      {children}
    </div>
  )
}
