// SAFE: derives only needed client-safe value from the server context

import { headers } from 'next/headers'
import { ClientWidget } from './client-widget'

export default async function Page() {
  const h = await headers()
  const userAgent = h.get('user-agent') ?? 'unknown'
  const isMobile = /Mobile/i.test(userAgent)
  return <ClientWidget isMobile={isMobile} />
}
