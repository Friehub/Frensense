// SAFE: User identity is derived from the authenticated session, not from request body parameters.

import { type Request, type Response, type NextFunction } from 'express'

export function exportData() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const userId = res.locals.userId
      res.json({ userData: { id: userId } })
    } catch (error) {
      next(error)
    }
  }
}
