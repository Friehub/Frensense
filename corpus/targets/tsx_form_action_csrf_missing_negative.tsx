// SAFE: server action that checks the Origin/Referer header and validates CSRF token

'use server'

import { cookies, headers } from 'next/headers'
import { redirect } from 'next/navigation'

export async function transfer(prevState: unknown, data: FormData) {
  const h = await headers()
  const origin = h.get('origin')
  const referer = h.get('referer')
  const allowed = new URL('http://localhost:3000')
  if (!origin?.startsWith(allowed.origin) && !referer?.startsWith(allowed.origin)) {
    throw new Error('CSRF check failed')
  }
  const token = data.get('csrf') as string
  const cookieToken = (await cookies()).get('csrf-token')?.value
  if (!token || token !== cookieToken) throw new Error('CSRF token mismatch')
  redirect('/success')
}
