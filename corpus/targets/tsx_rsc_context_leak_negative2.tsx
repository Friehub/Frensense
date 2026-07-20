// SAFE: server component renders conditionally on the server, no context values leaked to client

import { headers } from 'next/headers'
import { ClientWidget } from './client-widget'

export default async function Page() {
  const h = await headers()
  const userAgent = h.get('user-agent') ?? ''
  if (/bot/i.test(userAgent)) {
    return <div>Welcome, bot.</div>
  }
  return <ClientWidget />
}
