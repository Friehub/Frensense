// SAFE: User identity is derived from the authenticated session, not from request body parameters.

import { type Request, type Response, type NextFunction } from 'express'

class UserModel {
  static async findByPk(id: number) { return { id, username: 'test' } }
  async update(attrs: Record<string, any>) { return this }
}

export function updateProfile() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const userId = res.locals.userId
      const user = await UserModel.findByPk(userId)
      const saved = await user.update({ username: req.body.username })
      res.json(saved)
    } catch (error) {
      next(error)
    }
  }
}
