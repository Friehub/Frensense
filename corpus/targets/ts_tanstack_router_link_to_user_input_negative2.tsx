// SAFE: User input is mapped to an allowlist of valid route paths

import { Link } from '@tanstack/react-router'

const ROUTE_ALLOWLIST: Record<string, string> = {
  dashboard: '/dashboard',
  profile: '/profile',
  settings: '/settings',
}

export function DynamicLink({ routeKey }: { routeKey: string }) {
  const safePath = ROUTE_ALLOWLIST[routeKey] ?? '/'

  return (
    <Link to={safePath}>
      Go
    </Link>
  )
}
