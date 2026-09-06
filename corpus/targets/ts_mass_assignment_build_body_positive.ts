// [frensense]
// observation: The entire request body is passed directly to an ORM model build or create call, allowing mass assignment of arbitrary model attributes.
// impact: An attacker can set any model field (e.g., role, isAdmin, balance) by including it in the request body, bypassing intended access controls.
// improvement: Use explicit attribute allowlisting (e.g., Model.build({ name: req.body.name })) instead of passing the whole req.body.
// cwe: CWE-915
// cvss: 7.5
// owasp: A01:2021

import { type Request, type Response, type NextFunction } from 'express'

class ItemModel {
  static build(attrs: Record<string, any>) { return new ItemModel() }
  async save() { return this }
}

export function addItem() {
  return async (req: Request, res: Response, next: NextFunction) => {
    try {
      const item = await ItemModel.build(req.body).save()
      res.json({ status: 'success', data: item })
    } catch (error) {
      next(error)
    }
  }
}
