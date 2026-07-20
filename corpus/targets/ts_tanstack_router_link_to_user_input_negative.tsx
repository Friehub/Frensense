// SAFE: The `to` prop uses a static route path, not user input

import { Link } from '@tanstack/react-router'

export function StaticLink() {
  return (
    <Link to="/dashboard">
      Go to Dashboard
    </Link>
  )
}
