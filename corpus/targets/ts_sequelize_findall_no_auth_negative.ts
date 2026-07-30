// SAFE: The query is filtered by the authenticated user's identity before returning results.

import { type Request, type Response, type NextFunction } from 'express'

class OrderModel {
  static async findAll(opts?: any) { return [] }
}

export function allOrders() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const email = res.locals.email
      const orders = await OrderModel.findAll({ where: { email } })
      res.json({ status: 'success', data: orders })
    } catch (error) {
      next(error)
    }
  }
}
