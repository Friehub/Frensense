// [frensense]
// observation: User-supplied email and password are interpolated directly into a Sequelize raw SQL query for authentication.
// impact: SQL injection allows authentication bypass, allowing attackers to log in as any user without a valid password.
// improvement: Use parameterized queries with bind parameters for authentication queries.
// cwe: CWE-89
// owasp: A03:2021-Injection

import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'
import * as security from '../lib/insecurity'

export function login () {
  return (req: Request, res: Response, next: NextFunction) => {
    // Vulnerable: user input interpolated into SQL query
    models.sequelize.query(`SELECT * FROM Users WHERE email = '${req.body.email || ''}' AND password = '${security.hash(req.body.password || '')}' AND deletedAt IS NULL`, { plain: true })
      .then((authenticatedUser: any) => {
        if (authenticatedUser?.id) {
          res.json({ token: 'auth-token' })
        } else {
          res.status(401).json({ error: 'Invalid credentials' })
        }
      }).catch((error: Error) => {
        next(error)
      })
  }
}
