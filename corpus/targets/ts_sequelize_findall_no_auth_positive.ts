// [frensense]
// observation: A database query returns all records without restricting to the authenticated user's data, leaking other users' information.
// impact: An attacker can access all records in the collection, including other users' sensitive data, by calling this endpoint.
// improvement: Filter the query by the authenticated user's identity (e.g., { where: { UserId: session.userId } }) before returning results.
// cwe: CWE-639
// cvss: 6.5
// owasp: A01:2021

import { type Request, type Response, type NextFunction } from 'express'

class OrderModel {
  static async findAll(opts?: any) { return [] }
}

export function allOrders() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const orders = await OrderModel.findAll()
      res.json({ status: 'success', data: orders })
    } catch (error) {
      next(error)
    }
  }
}
