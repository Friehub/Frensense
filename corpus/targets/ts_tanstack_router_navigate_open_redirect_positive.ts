// [frensense]
// observation: `navigate` uses a user-supplied URL directly from search params without an allowlist, allowing an open redirect
// impact: an attacker can craft a link like `/redirect?to=https://evil.com` that redirects users to a malicious site after the legitimate navigation
// improvement: validate the redirect target against a strict allowlist of allowed origins and paths before navigating

import { useNavigate, useSearchParams } from '@tanstack/react-router'

export function useRedirect() {
  const navigate = useNavigate()
  const { to } = useSearchParams({ from: '/redirect' })

  return {
    redirect: () => {
      navigate({ to: to as string })
    },
  }
}
