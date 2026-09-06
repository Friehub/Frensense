// SAFE: User input is passed to the ORM's findOne method which parameterizes the query internally.

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize, Model } from 'sequelize'

class UserModel extends Model {}
const models = { sequelize: new Sequelize('sqlite::memory:'), UserModel }

export function queryLogin() {
  return (req: Request, res: Response, next: NextFunction) => {
    const email = req.body.email || ''
    UserModel.findOne({ where: { email, deletedAt: null } })
      .then((user: any) => {
        res.json(user)
      }).catch((error: Error) => {
        next(error)
      })
  }
}

