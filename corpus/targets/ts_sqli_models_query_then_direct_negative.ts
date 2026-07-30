// SAFE: User input is passed to the ORM's findOne method which parameterizes the query internally.

import { Router } from 'express'
import { Request, Response, NextFunction } from 'express'
import { Sequelize, Model } from 'sequelize'

class UserModel extends Model {}
const models = { sequelize: new Sequelize('sqlite::memory:'), UserModel }
const router = Router()

router.post('/api/login', (req: Request, res: Response, next: NextFunction) => {
  const email = req.body.email || ''
  UserModel.findOne({ where: { email, deletedAt: null } })
    .then((user: any) => {
      res.json(user)
    }).catch((error: Error) => {
      next(error)
    })
})

export default router
