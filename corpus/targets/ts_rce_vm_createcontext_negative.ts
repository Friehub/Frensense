// SAFE: User input is parsed as structured JSON data instead of being evaluated in a VM context.

import { type Request, type Response, type NextFunction } from 'express'

export function b2bOrder() {
  return ({ body }: Request, res: Response, next: NextFunction) => {
    try {
      const orderLinesData = body.orderLinesData || ''
      JSON.parse(orderLinesData)
      res.json({ cid: body.cid })
    } catch (err) {
      next(err)
    }
  }
}
