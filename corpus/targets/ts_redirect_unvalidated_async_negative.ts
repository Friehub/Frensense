// SAFE: Redirect destination is validated against an allowlist before redirecting.

import { type Request, type Response, type NextFunction } from 'express'

const ALLOWED_REDIRECTS = ['/profile', '/dashboard', '/login']

export function performRedirect() {
  return (req: Request, res: Response, next: NextFunction) => {
    const toUrl = req.query.to as string
    if (ALLOWED_REDIRECTS.includes(toUrl)) {
      res.redirect(toUrl)
    } else {
      res.status(406)
      next(new Error('Unrecognized target URL for redirect: ' + toUrl))
    }
  }
}
