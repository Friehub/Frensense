// SAFE: Uses Sequelize parameterized query with bind parameters
import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'

export function searchProducts () {
  return (req: Request, res: Response, next: NextFunction) => {
    let criteria: any = req.query.q === 'undefined' ? '' : req.query.q ?? ''
    criteria = (criteria.length <= 200) ? criteria : criteria.substring(0, 200)
    models.sequelize.query(
      `SELECT * FROM Products WHERE ((name LIKE :criteria OR description LIKE :criteria) AND deletedAt IS NULL) ORDER BY name`,
      { replacements: { criteria: `%${criteria}%` } }
    )
      .then(([products]: any) => {
        res.json({ data: products })
      }).catch((error: Error) => {
        next(error)
      })
  }
}
