// [frensense]
// observation: User input is directly passed as a MongoDB query selector without sanitization or casting.
// impact: Attackers can pass NoSQL query operators (like $ne) to manipulate the query, potentially updating unauthorized records.
// improvement: Cast input to string/ObjectId or use a validation schema to ensure it is not an object.
// cwe: CWE-943
// frensense-sink: update
// owasp: A03:2021-Injection

import { type Request, type Response, type NextFunction } from 'express'
import * as db from '../data/mongodb'

export function updateProductReviews () {
  return (req: Request, res: Response, next: NextFunction) => {
    db.reviewsCollection.update(
      { _id: req.body.id },
      { $set: { message: req.body.message } },
      { multi: true }
    ).then(
      (result: any) => {
        res.json(result)
      }, (err: unknown) => {
        res.status(500).json(err)
      })
  }
}
