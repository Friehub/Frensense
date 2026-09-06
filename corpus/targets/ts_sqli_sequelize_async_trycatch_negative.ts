// SAFE: User input is passed as a bind parameter instead of being interpolated into the SQL string.

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize } from 'sequelize'
const sequelize = new Sequelize('sqlite::memory:')

export function findUser() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const email = req.body.email
      const users = await sequelize.query('SELECT * FROM Users WHERE email = ?', { replacements: [email] })
      res.json(users)
    } catch (error) {
      next(error)
    }
  }
}
