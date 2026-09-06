// SAFE: User input is validated and sanitized before being used in a query.

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const db = new MongoClient('mongodb://localhost:27017').db('test')

export function trackOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = String(req.params.id).replace(/[^\w-]+/g, '')
    db.collection('orders').find({ orderId: id }).toArray().then((order: any) => {
      res.json(order)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
