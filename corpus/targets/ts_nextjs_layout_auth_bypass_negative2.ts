// SAFE: each admin route group runs its own auth check in a shared guard component

import { getSession } from '@/lib/session'
import { redirect } from 'next/navigation'
import { ReactNode } from 'react'

export default async function AdminLayout({ children }: { children: ReactNode }) {
  const session = await getSession()
  if (!session) redirect('/login')
  return <div>{children}</div>
}

export async function generateMetadata() {
  const session = await getSession()
  if (!session) return {}
  return { title: 'Admin' }
}
