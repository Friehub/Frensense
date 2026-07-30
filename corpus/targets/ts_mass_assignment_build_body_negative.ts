// SAFE: Only explicitly allowlisted fields are passed to the ORM build call.

import { type Request, type Response, type NextFunction } from 'express'

class ItemModel {
  static build(attrs: Record<string, any>) { return new ItemModel() }
  async save() { return this }
}

export function addItem() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const item = await ItemModel.build({ name: req.body.name }).save()
      res.json({ status: 'success', data: item })
    } catch (error) {
      next(error)
    }
  }
}
