// [frensense]
// observation: User-controlled input is interpolated into a raw Sequelize query without parameterization, enabling SQL injection.
// impact: An attacker can execute arbitrary SQL commands, exfiltrate data, or bypass authentication.
// improvement: Use parameterized queries with replacements or bind parameters instead of string interpolation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize } from 'sequelize'
const sequelize = new Sequelize('sqlite::memory:')

export function findUser() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const email = req.body.email
      const users = await sequelize.query(`SELECT * FROM Users WHERE email = '${email}'`)
      res.json(users)
    } catch (error) {
      next(error)
    }
  }
}
