// SAFE: Uses Sequelize parameterized query for authentication
import { type Request, type Response, type NextFunction } from 'express'
import * as models from '../models/index'
import * as security from '../lib/insecurity'

export function login () {
  return (req: Request, res: Response, next: NextFunction) => {
    models.sequelize.query(
      'SELECT * FROM Users WHERE email = :email AND password = :password AND deletedAt IS NULL',
      { replacements: { email: req.body.email || '', password: security.hash(req.body.password || '') }, plain: true }
    )
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
