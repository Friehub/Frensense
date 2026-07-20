// SAFE: validates that the userId in the form matches the authenticated user

'use server'

import { redirect } from 'next/navigation'
import { cookies } from 'next/headers'
import { verifySession } from '@/lib/session'

export async function updateProfile(data: FormData) {
  const token = (await cookies()).get('session')?.value
  const session = await verifySession(token)
  const formUserId = data.get('userId') as string
  if (formUserId !== session.userId) throw new Error('User ID mismatch')
  const name = data.get('name') as string
  await db.user.update({ where: { id: formUserId }, data: { name } })
  redirect('/profile')
}
