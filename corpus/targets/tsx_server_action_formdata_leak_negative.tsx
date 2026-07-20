// SAFE: derives userId and role from the server-side session, not from the form

'use server'

import { redirect } from 'next/navigation'
import { cookies } from 'next/headers'
import { verifySession } from '@/lib/session'

export async function updateProfile(data: FormData) {
  const token = (await cookies()).get('session')?.value
  const session = await verifySession(token)
  const name = data.get('name') as string
  await db.user.update({ where: { id: session.userId }, data: { name } })
  redirect('/profile')
}
