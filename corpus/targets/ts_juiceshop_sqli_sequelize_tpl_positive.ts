// [frensense]
// observation: User input is directly interpolated into a Sequelize raw SQL query using template literals without parameterization.
// impact: Attackers can inject arbitrary SQL commands through user input, potentially reading, modifying, or deleting database records.
// improvement: Use parameterized queries or Sequelize's bind parameter syntax instead of string interpolation.
// cwe: CWE-89
// frensense-sink: query
// owasp: A03:2021-Injection

import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'

export function searchProducts () {
  return (req: Request, res: Response, next: NextFunction) => {
    let criteria: any = req.query.q === 'undefined' ? '' : req.query.q ?? ''
    criteria = (criteria.length <= 200) ? criteria : criteria.substring(0, 200)
    models.sequelize.query(`SELECT * FROM Products WHERE ((name LIKE '%${criteria}%' OR description LIKE '%${criteria}%') AND deletedAt IS NULL) ORDER BY name`)
      .then(([products]: any) => {
        res.json({ data: products })
      }).catch((error: Error) => {
        next(error)
      })
  }
}
