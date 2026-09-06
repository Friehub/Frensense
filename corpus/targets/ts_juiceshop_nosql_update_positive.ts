// [frensense]
// observation: User input is directly passed as the query object in a database update operation without type validation, allowing NoSQL injection.
// impact: An attacker can supply a query operator like {$ne: null} to bypass single-document constraints and update unintended records.
// improvement: Validate or cast user input to a string/ObjectId before passing it to database queries.
// cwe: CWE-943
// cvss: 8.5
// owasp: A03:2021
// frensense-sink: update

import { type Request, type Response, type NextFunction } from 'express'
import { MongoClient } from 'mongodb'

const client = new MongoClient('mongodb://localhost:27017')
const db = client.db('test')

export function updateRecords() {
  return (req: Request, res: Response, next: NextFunction) => {
    db.collection('reviews').update(
      { _id: req.body.id },
      { $set: { message: req.body.message } },
      { multi: true }
    ).then((result: any) => {
      res.json(result)
    }).catch((err: Error) => {
      res.status(500).json(err)
    })
  }
}
