// SAFE: User input is sanitized to remove special characters before being used in a $where clause.

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const client = new MongoClient('mongodb://localhost:27017')
const db = client.db('test')

export function findItems() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = String(req.params.id).replace(/[^\w-]+/g, '')
    db.collection('items').find({ orderId: id }).toArray().then((items: any) => {
      res.json(items)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
