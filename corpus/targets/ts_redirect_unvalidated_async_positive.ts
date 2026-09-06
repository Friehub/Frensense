// [frensense]
// observation: The redirect destination is taken directly from a query parameter without validating that it points to the same origin.
// impact: An attacker can craft a link that redirects users to a phishing site after a successful login.
// improvement: Validate that the redirect target is a relative path or belongs to an allowlist before redirecting.
// cwe: CWE-601
// cvss: 6.1
// owasp: A01:2021
// runtime_probe: redirect

import { type Request, type Response, type NextFunction } from 'express'

export function performRedirect() {
  return (req: Request, res: Response, next: NextFunction) => {
    const toUrl = req.query.to as string
    res.redirect(toUrl)
  }
}
