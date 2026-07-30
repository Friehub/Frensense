// SAFE: User input is passed to the ORM's findOne method which parameterizes the query internally.

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize } from 'sequelize'
import { Model } from 'sequelize'
const sequelize = new Sequelize('sqlite::memory:')

class UserModel extends Model {}

export function findUser() {
  return (req: Request, res: Response, next: NextFunction) => {
    const email = req.body.email || ''
    UserModel.findOne({ where: { email, active: 1 } })
      .then((user: any) => {
        res.json(user)
      }).catch((error: Error) => {
        next(error)
      })
  }
}


