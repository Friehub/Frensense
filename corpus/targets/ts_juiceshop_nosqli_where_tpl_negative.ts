// SAFE: Uses MongoDB query operators instead of $where with string interpolation
import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'

export function updateProductReviews () {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id
    const review = req.body.review
    models.Product.updateOne(
      { _id: id, $expr: { $lt: [{ $size: "$reviews" }, 3] } },
      { $push: { reviews: { message: review } } }
    )
      .then(() => {
        res.json({ success: true })
      }).catch((error: Error) => {
        next(error)
      })
  }
}
