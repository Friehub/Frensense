// [frensense]
// observation: A database lookup uses a user-controlled identifier from the request body instead of the authenticated session, enabling IDOR.
// impact: An attacker can tamper with the identifier parameter to access another user's data without authorization.
// improvement: Derive the user's identity from the authenticated session instead of from request body parameters.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021

import { type Request, type Response, type NextFunction } from 'express'

class UserModel {
  static async findByPk(id: number) { return { id, username: 'test' } }
  async update(attrs: Record<string, any>) { return this }
}

export function updateProfile() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const user = await UserModel.findByPk(req.body.UserId)
      const saved = await user.update({ username: req.body.username })
      res.json(saved)
    } catch (error) {
      next(error)
    }
  }
}
