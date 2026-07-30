// SAFE: User input is passed as a bind parameter instead of being interpolated into the SQL string.

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize } from 'sequelize'
const sequelize = new Sequelize('sqlite::memory:')

export function login() {
  return (req: Request, res: Response, next: NextFunction) => {
    const email = req.body.email
    sequelize.query('SELECT * FROM Users WHERE email = ?', { replacements: [email] }).then(([users]: any) => {
      res.json(users)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
