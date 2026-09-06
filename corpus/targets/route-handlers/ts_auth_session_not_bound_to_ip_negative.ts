// SAFE: Session bound to IP address at login, verified on each request.
import { Request, Response, NextFunction } from 'express';
import session from 'express-session';

declare module 'express-session' {
  interface SessionData {
    userId: number;
    boundIp: string;
  }
}

function getClientIp(req: Request): string {
  return req.ip || req.socket.remoteAddress || 'unknown';
}

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password } = req.body;
  if (username === 'admin' && password === 'secret') {
    req.session.userId = 1;
    req.session.boundIp = getClientIp(req);
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
  if (req.session.boundIp && req.session.boundIp !== getClientIp(req)) {
    req.session.destroy(() => {});
    res.status(401).json({ error: 'session hijacked' });
    return;
  }
  next();
}
