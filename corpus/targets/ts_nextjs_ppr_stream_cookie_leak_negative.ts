// SAFE: Cookie errors are caught and replaced with a generic message, with details logged server-side only

import { cookies } from 'next/headers'
import { redirect } from 'next/navigation'

export default async function AppLayout({ children }: { children: React.ReactNode }) {
  let sessionCookie
  try {
    sessionCookie = (await cookies()).get('session')
  } catch (err) {
    console.error('Cookie parse error:', err)
    redirect('/login')
  }

  if (!sessionCookie) {
    redirect('/login')
  }

  return <div>{children}</div>
}
