// [frensense]
// observation: User-controlled input is passed to vm.createContext() and vm.runInContext() for evaluation, enabling arbitrary code execution.
// impact: An attacker can execute arbitrary JavaScript code on the server by including malicious code in the request body.
// improvement: Avoid using vm.createContext() with user input. Use safe evaluation libraries or parse the input as structured data instead.
// cwe: CWE-94
// cvss: 9.8
// owasp: A03:2021

import vm from 'node:vm'
import { type Request, type Response, type NextFunction } from 'express'

export function b2bOrder() {
  return ({ body }: Request, res: Response, next: NextFunction) => {
    try {
      const orderLinesData = body.orderLinesData || ''
      const sandbox = { orderLinesData }
      vm.createContext(sandbox)
      vm.runInContext('JSON.parse(orderLinesData)', sandbox, { timeout: 2000 })
      res.json({ cid: body.cid })
    } catch (err) {
      next(err)
    }
  }
}
