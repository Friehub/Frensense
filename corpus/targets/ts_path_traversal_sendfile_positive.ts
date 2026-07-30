// [frensense]
// observation: A user-controlled file name from URL parameters is passed directly to res.sendFile() without path validation, enabling path traversal.
// impact: An attacker can read arbitrary files on the server by including ../ sequences in the file parameter.
// improvement: Validate that the file path is within an allowlisted directory and reject paths containing ../ or absolute paths.
// cwe: CWE-22
// cvss: 7.5
// owasp: A01:2021

import path from 'node:path'
import { type Request, type Response, type NextFunction } from 'express'

export function serveFile() {
  return (req: Request, res: Response, next: NextFunction) => {
    const file = req.params.file as string
    res.sendFile(path.resolve('ftp/', file))
  }
}
