// [frensense]
// observation: A database update operation uses a user-controlled identifier without verifying ownership, enabling unauthorized data modification.
// impact: An attacker can modify another user's data by providing a different identifier in the request.
// improvement: Verify that the record belongs to the authenticated user before performing the update.
// cwe: CWE-639
// cvss: 7.5
// owasp: A01:2021

import { type Request, type Response, type NextFunction } from 'express'

class BasketModel {
  static async findByPk(id: number) { return { id, coupon: '' } }
  async update(attrs: Record<string, any>) { return this }
}

export function updateBasket() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const basket = await BasketModel.findByPk(req.params.id)
      await basket.update({ coupon: req.body.coupon })
      res.json({ status: 'success' })
    } catch (error) {
      next(error)
    }
  }
}
