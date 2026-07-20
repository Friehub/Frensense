// SAFE: authentication checked via getSession before returning user data

import type { LoaderFunctionArgs } from '@remix-run/node'
import { json, redirect } from '@remix-run/node'
import { db } from '~/db.server'
import { getSession } from '~/sessions'

export async function loader({ params, request }: LoaderFunctionArgs) {
  const session = await getSession(request.headers.get('Cookie'))
  const currentUserId = session.get('userId')

  if (!currentUserId) {
    throw redirect('/login')
  }

  const userId = params.userId
  const user = await db.user.findUnique({
    where: { id: userId },
    select: { id: true, email: true, billingAddress: true, ssn: true }
  })

  return json(user)
}
