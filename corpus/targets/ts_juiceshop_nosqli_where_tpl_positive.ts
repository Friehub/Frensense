// [frensense]
// observation: User input is interpolated into a MongoDB $where query operator using template literals.
// impact: NoSQL injection allows attackers to execute arbitrary JavaScript code on the MongoDB server, potentially reading or modifying data.
// improvement: Use MongoDB parameterized queries or sanitize input before using in $where clauses.
// cwe: CWE-943
// owasp: A03:2021-Injection

import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'

export function updateProductReviews () {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id
    const review = req.body.review
    models.Product.update(
      { $where: `this._id == '${id}' && this.reviews.length < 3` },
      { $push: { reviews: { message: review } } }
    )
      .then(() => {
        res.json({ success: true })
      }).catch((error: Error) => {
        next(error)
      })
  }
}
