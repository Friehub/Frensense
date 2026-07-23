// SAFE: redirect target is validated against an origin check and path prefix allowlist

import { useNavigate, useSearchParams } from '@tanstack/react-router'

const ALLOWED_PATHS = ['/dashboard', '/settings', '/profile'] as const

function isValidRedirect(raw: unknown): raw is string {
  if (typeof raw !== 'string') return false
  try {
    const url = new URL(raw, window.location.origin)
    if (url.origin !== window.location.origin) return false
    return ALLOWED_PATHS.some((p) => url.pathname === p || url.pathname.startsWith(p + '/'))
  } catch {
    return ALLOWED_PATHS.includes(raw as typeof ALLOWED_PATHS[number])
  }
}

export function useRedirect() {
  const navigate = useNavigate()
  const { to } = useSearchParams({ from: '/redirect' })

  return {
    redirect: () => {
      const target = isValidRedirect(to) ? to : '/dashboard'
      navigate({ to: target })
    },
  }
}
