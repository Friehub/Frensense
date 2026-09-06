// SAFE: User input is encoded before being included in the HTML response, preventing XSS.

import { type Request, type Response, type NextFunction } from 'express'

function escapeHtml(str: string): string {
  return str.replace(/&/g, '&amp;').replace(/</g, '&lt;').replace(/>/g, '&gt;').replace(/"/g, '&quot;')
}

export function trackOrder() {
  return (req: Request, res: Response, next: NextFunction) => {
    const id = req.params.id as string
    res.send(`<html><body>Tracking order ${escapeHtml(id)}</body></html>`)
  }
}
