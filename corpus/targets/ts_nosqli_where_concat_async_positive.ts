// [frensense]
// observation: User-controlled input is concatenated into a MongoDB $where clause, enabling NoSQL injection via JavaScript expression.
// impact: An attacker can inject arbitrary JavaScript into the $where expression, exfiltrating data or executing operations on the database server.
// improvement: Avoid $where with string interpolation. Use typed query filters or validate that the input does not contain MongoDB operator syntax before interpolation.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const db = new MongoClient('mongodb://localhost:27017').db('test')

export function trackOrder() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const id = req.params.id as string
      const order = await db.collection('orders').find({ $where: `this.orderId === '${id}'` }).toArray()
      res.json(order)
    } catch (error) {
      next(error)
    }
  }
}
