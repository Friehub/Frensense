// SAFE: User input is parsed as structured JSON data instead of being evaluated in a VM context.

import { type Request, type Response, type NextFunction } from 'express'

export function executeOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    try {
      const orderData = req.body.orderLinesData || ''
      const parsed = JSON.parse(orderData)
      res.json({ status: 'success', data: parsed })
    } catch (err) {
      next(err)
    }
  }
}
