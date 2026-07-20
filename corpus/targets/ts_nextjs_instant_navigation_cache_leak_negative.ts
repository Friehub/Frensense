// SAFE: User-specific pages disable prefetch via prefetch={false}, preventing instant navigation cache capture

import Link from 'next/link'

export default function Nav() {
  return (
    <nav>
      <Link href="/dashboard" prefetch={false}>Dashboard</Link>
      <Link href="/inbox" prefetch={false}>Inbox</Link>
    </nav>
  )
}
