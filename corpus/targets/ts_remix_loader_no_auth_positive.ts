// [frensense]
// observation: Remix route loader returns user data without checking authentication
// impact: any user (including unauthenticated) can access other users' private data via the loader response
// improvement: verify authentication via session or requireUserId before returning data
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import type { LoaderFunctionArgs } from '@remix-run/node'
import { json } from '@remix-run/node'
import { db } from '~/db.server'

export async function loader({ params }: LoaderFunctionArgs) {
  const userId = params.userId
  const user = await db.user.findUnique({
    where: { id: userId },
    select: { id: true, email: true, billingAddress: true, ssn: true }
  })

  return json(user)
}
