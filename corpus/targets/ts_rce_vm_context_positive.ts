// [frensense]
// observation: User-controlled input is passed directly to vm.runInContext() for evaluation, enabling arbitrary code execution.
// impact: An attacker can execute arbitrary JavaScript code on the server by including malicious code in the request body.
// improvement: Avoid using vm.runInContext() with user input. Use safe evaluation libraries or parse the input as structured data instead.
// cwe: CWE-94
// cvss: 9.8
// owasp: A03:2021

import vm from 'node:vm'
import { type Request, type Response, type NextFunction } from 'express'

export function executeOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    try {
      const orderData = req.body.orderLinesData || ''
      const sandbox = { orderLinesData: orderData }
      vm.createContext(sandbox)
      vm.runInContext('JSON.parse(orderLinesData)', sandbox, { timeout: 2000 })
      res.json({ status: 'success' })
    } catch (err) {
      next(err)
    }
  }
}
