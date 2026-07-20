// [frensense]
// observation: `<Link to={userInput}>` uses user-controlled input directly as the `to` prop without validation or an allowlist, enabling navigation to arbitrary URLs
// impact: open redirect — an attacker can inject a malicious URL (e.g., `https://evil.com`) as the `to` prop, redirecting users to a phishing site when they click the link
// improvement: validate user-controlled paths against an allowlist of valid routes, or use a mapping object to convert user input to safe route paths

import { Link } from '@tanstack/react-router'
import { useSearchParams } from '@tanstack/react-router'

export function DynamicLink() {
  const { redirectTo } = useSearchParams({ from: '/redirect' })

  return (
    <Link to={redirectTo as string}>
      Click here
    </Link>
  )
}
