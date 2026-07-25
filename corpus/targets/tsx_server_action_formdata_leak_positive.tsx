// [frensense]
// observation: server action receives FormData from a form containing hidden fields with internal user data (token, role, userId) — these hidden fields are exposed in the HTML source and the action trusts them
// impact: an attacker can modify hidden field values in the FormData before submission, escalating privileges or impersonating users
// improvement: derive sensitive values server-side from the session, never from client-supplied hidden fields
// cwe: CWE-200
// cvss: 5.3
// owasp: 
// severity: Medium

'use server'

import { redirect } from 'next/navigation'

export async function updateProfile(data: FormData) {
  const userId = data.get('userId') as string
  const role = data.get('role') as string
  const name = data.get('name') as string
  await db.user.update({ where: { id: userId }, data: { name, role } })
  redirect('/profile')
}
