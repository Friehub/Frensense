// [frensense]
// observation: User-controlled input from URL parameters is reflected in the HTTP response without escaping, enabling reflected XSS.
// impact: An attacker can execute arbitrary JavaScript in the victim's browser by crafting a malicious link.
// improvement: Escape or encode user-controlled data before including it in HTML responses. Use Content-Type: application/json for API responses.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// runtime_probe: xss

import { type Request, type Response, type NextFunction } from 'express'

export function trackOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id as string
    res.send(`<html><body>Tracking order ${id}</body></html>`)
  }
}
