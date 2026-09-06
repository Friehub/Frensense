// SAFE: File name is validated to be within an allowlisted directory and rejects path traversal sequences.

import path from 'node:path'
import { type Request, type Response, type NextFunction } from 'express'

const ALLOWED_DIR = path.resolve('ftp/')

export function serveFile() {
  return (req: Request, res: Response, next: NextFunction) => {
    const file = req.params.file as string
    if (file.includes('..') || file.includes('/')) {
      res.status(403)
      next(new Error('File names cannot contain path separators!'))
      return
    }
    res.sendFile(path.join(ALLOWED_DIR, file))
  }
}
