// [frensense]
// observation: A sensitive identifier (UserId) is taken directly from the request body instead of the authenticated session, enabling Insecure Direct Object Reference.
// impact: An attacker can tamper with the UserId parameter in the request body to access another user's data without authorization.
// improvement: Always derive the current user's identity from the authenticated session rather than from request body parameters.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021

import { type Request, type Response, type NextFunction } from 'express'

export function exportData() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const userId = req.body.UserId
      res.json({ userData: { id: userId } })
    } catch (error) {
      next(error)
    }
  }
}
