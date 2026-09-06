// [frensense]
// observation: User-controlled input is concatenated into a MongoDB $where clause in a .then() chain, enabling NoSQL injection.
// impact: An attacker can inject arbitrary JavaScript into the $where expression, exfiltrating data or executing operations on the database server.
// improvement: Avoid $where with string interpolation. Use typed query filters instead.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const db = new MongoClient('mongodb://localhost:27017').db('test')

export function trackOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id as string
    db.collection('orders').find({ $where: `this.orderId === '${id}'` }).toArray().then((order: any) => {
      res.json(order)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
