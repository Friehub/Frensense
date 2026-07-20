// [frensense]
// observation: Server Component fetches data with auth token and passes full response to client component — sensitive fields included
// impact: auth token usage is safe server-side but the RSC serialization leaks sensitive user fields (ssn, apiKey) to the client payload
// improvement: strip sensitive fields before passing to client component or use a dedicated API route

import { cookies } from 'next/headers'
import { ClientProfile } from './client-profile'

export default async function Dashboard() {
  const token = (await cookies()).get('session')?.value
  const res = await fetch('https://internal-api/user/profile', {
    headers: { Authorization: `Bearer ${token}` }
  })
  const user: { name: string; ssn: string; apiKey: string; email: string } = await res.json()
  return <ClientProfile user={user} />
}
