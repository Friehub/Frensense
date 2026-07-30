// SAFE: The record ownership is verified against the authenticated user before performing the update.

import { type Request, type Response, type NextFunction } from 'express'

class BasketModel {
  static async findByPk(id: number) { return { id, coupon: '' } }
  async update(attrs: Record<string, any>) { return this }
}

export function updateBasket() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const basket = await BasketModel.findByPk(req.params.id)
      if (basket.userId !== res.locals.userId) {
        res.status(403).json({ error: 'Not authorized' })
        return
      }
      await basket.update({ coupon: req.body.coupon })
      res.json({ status: 'success' })
    } catch (error) {
      next(error)
    }
  }
}
