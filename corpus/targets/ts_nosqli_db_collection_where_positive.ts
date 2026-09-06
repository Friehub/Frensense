// [frensense]
// observation: User-controlled input is concatenated into a MongoDB $where clause via db.collection.find(), enabling NoSQL injection.
// impact: An attacker can inject arbitrary JavaScript into the $where expression, exfiltrating data or executing operations on the database server.
// improvement: Avoid $where with string concatenation. Use typed query filters instead.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'
const client = new MongoClient('mongodb://localhost:27017')
const db = client.db('test')

export function findItems() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id as string
    db.collection('items').find({ $where: `this.id === '${id}'` }).toArray().then((items: any) => {
      res.json(items)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
