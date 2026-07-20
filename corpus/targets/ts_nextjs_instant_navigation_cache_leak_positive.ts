// [frensense]
// observation: Instant Navigations in Next.js 16.3 pre-fetch and cache page content based on `<Link>` visibility. When a page contains user-specific data and the instant navigation caches the RSC payload, the cached response is served to subsequent users who navigate to the same URL.
// impact: An attacker can pre-fetch a protected dashboard page while logged in, then the cached payload is served to an unauthenticated user, leaking sensitive user data.
// improvement: Mark user-specific pages as `dynamic = 'force-dynamic'` or add user-specific cache tags to prevent cross-user instant navigation cache hits.

import Link from 'next/link'

export default function Nav() {
  return (
    <nav>
      <Link href="/dashboard" prefetch={true}>Dashboard</Link>
      <Link href="/inbox">Inbox</Link>
    </nav>
  )
}
