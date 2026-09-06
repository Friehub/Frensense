// SAFE: User input is validated and sanitized before being used in a $where clause, preventing injection.

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const db = new MongoClient('mongodb://localhost:27017').db('test')

export function trackOrder() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const id = String(req.params.id).replace(/[^\w-]+/g, '')
      const order = await db.collection('orders').find({ orderId: id }).toArray()
      res.json(order)
    } catch (error) {
      next(error)
    }
  }
}
