// SAFE: strips sensitive fields before passing data to client component

import { cookies } from 'next/headers'
import { ClientProfile } from './client-profile'

export default async function Dashboard() {
  const token = (await cookies()).get('session')?.value
  const res = await fetch('https://internal-api/user/profile', {
    headers: { Authorization: `Bearer ${token}` }
  })
  const user: { name: string; ssn: string; apiKey: string; email: string } = await res.json()
  const { ssn, apiKey, ...safeUser } = user
  return <ClientProfile user={safeUser} />
}
