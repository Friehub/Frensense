// SAFE: requireUserId helper used; sensitive fields excluded from select

import type { LoaderFunctionArgs } from '@remix-run/node'
import { json } from '@remix-run/node'
import { db } from '~/db.server'
import { requireUserId } from '~/auth.server'

export async function loader({ params, request }: LoaderFunctionArgs) {
  const currentUserId = await requireUserId(request)

  const userId = params.userId
  if (userId !== currentUserId) {
    throw new Response('Forbidden', { status: 403 })
  }

  const user = await db.user.findUnique({
    where: { id: userId },
    select: { id: true, email: true }
  })

  return json(user)
}
