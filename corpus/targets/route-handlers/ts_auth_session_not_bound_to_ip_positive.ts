// [frensense]
// observation: Session token is not bound to the IP address of the authenticated client. An attacker who steals the session cookie can use it from any IP address.
// impact: Session hijacking — stolen cookies from XSS, network sniffing, or side-channel attacks can be reused from attacker-controlled infrastructure without restriction.
// improvement: Bind session to the IP address at login and verify on each request, with grace period for legitimate IP changes.

import { Request, Response, NextFunction } from 'express';
import session from 'express-session';

declare module 'express-session' {
  interface SessionData {
    userId: number;
  }
}

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password } = req.body;
  if (username === 'admin' && password === 'secret') {
    req.session.userId = 1;
    res.json({ ok: true });
  } else {
    res.status(401).json({ error: 'invalid credentials' });
  }
}

export function requireAuth(req: Request, res: Response, next: NextFunction): void {
  if (!req.session.userId) {
    res.status(401).json({ error: 'unauthorized' });
    return;
  }
  next();
}
