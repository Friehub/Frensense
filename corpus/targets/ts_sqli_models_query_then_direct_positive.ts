// [frensense]
// observation: User-controlled input from request body is interpolated into a raw Sequelize query via models.sequelize.query() with template literals, enabling SQL injection.
// impact: An attacker can execute arbitrary SQL commands, exfiltrate data, or bypass authentication.
// improvement: Use parameterized queries with replacements or bind parameters instead of string interpolation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import { Router } from 'express'
import { Request, Response, NextFunction } from 'express'
import { Sequelize, Model } from 'sequelize'

class UserModel extends Model {}
const models = { sequelize: new Sequelize('sqlite::memory:'), UserModel }
const router = Router()

router.post('/api/login', (req: Request, res: Response, next: NextFunction) => {
  const email = req.body.email || ''
  models.sequelize.query(`SELECT * FROM Users WHERE email = '${email}' AND deletedAt IS NULL`, { model: UserModel, plain: true })
    .then((user: any) => {
      res.json(user)
    }).catch((error: Error) => {
      next(error)
    })
})

export default router
