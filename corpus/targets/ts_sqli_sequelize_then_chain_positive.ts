// [frensense]
// observation: User-controlled input is interpolated into a raw Sequelize query via .then() chain without parameterization, enabling SQL injection.
// impact: An attacker can execute arbitrary SQL commands, exfiltrate data, or bypass authentication.
// improvement: Use parameterized queries with replacements instead of string interpolation.
// cwe: CWE-89
// cvss: 9.8
// owasp: A03:2021
// runtime_probe: sqli

import { type Request, type Response, type NextFunction } from 'express'
import { Sequelize } from 'sequelize'
const sequelize = new Sequelize('sqlite::memory:')

export function login() {
  return (req: Request, res: Response, next: NextFunction) => {
    const email = req.body.email
    sequelize.query(`SELECT * FROM Users WHERE email = '${email}'`).then(([users]: any) => {
      res.json(users)
    }).catch((error: Error) => {
      next(error)
    })
  }
}
