// SAFE: fetches via API route that sanitizes the response

import { ClientProfile } from './client-profile'

export default async function Dashboard() {
  const res = await fetch('https://internal-api/user/safe-profile')
  const user: { name: string; email: string } = await res.json()
  return <ClientProfile user={user} />
}
