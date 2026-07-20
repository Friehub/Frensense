// SAFE: the redirect target is validated against a strict allowlist before navigating

import { useNavigate, useSearchParams } from '@tanstack/react-router'

const ALLOWED_REDIRECTS = ['/dashboard', '/settings', '/profile', '/items'] as const

function isAllowedRedirect(path: string): path is typeof ALLOWED_REDIRECTS[number] {
  return ALLOWED_REDIRECTS.includes(path as typeof ALLOWED_REDIRECTS[number])
}

export function useRedirect() {
  const navigate = useNavigate()
  const { to } = useSearchParams({ from: '/redirect' })

  return {
    redirect: () => {
      const safePath = isAllowedRedirect(to as string) ? to : '/dashboard'
      navigate({ to: safePath })
    },
  }
}
