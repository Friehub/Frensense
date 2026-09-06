// SAFE: server component fetches data and passes it to client component

import { cookies } from 'next/headers'
import { ClientProfile } from './client-profile'

export default async function ProfilePage() {
  const token = (await cookies()).get('session')?.value
  const res = await fetch('http://localhost:3000/api/internal/profile', {
    headers: { Cookie: `session=${token}` }
  })
  const data = await res.json()
  return <ClientProfile data={data} />
}
