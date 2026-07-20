// SAFE: Dashboard page is marked as dynamic to prevent static RSC payload caching

export const dynamic = 'force-dynamic'

import Link from 'next/link'

export default function Nav() {
  return (
    <nav>
      <Link href="/dashboard" prefetch={true}>Dashboard</Link>
      <Link href="/inbox" prefetch={true}>Inbox</Link>
    </nav>
  )
}
