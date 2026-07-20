// [frensense]
// observation: server-side `headers()` and `cookies()` return values are passed through to a client component as props
// impact: the RSC serialization embeds the raw header/cookie values into the client payload, potentially exposing tokens
// improvement: use server-side logic to derive only what the client needs; never pass request context directly

import { headers, cookies } from 'next/headers'
import { ClientWidget } from './client-widget'

export default async function Page() {
  const h = await headers()
  const userAgent = h.get('user-agent')
  const token = (await cookies()).get('session')?.value
  return <ClientWidget userAgent={userAgent} sessionToken={token} />
}
