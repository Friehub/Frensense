// SAFE: Session bound to IP via session data, with logged notification on mismatch.
import { Request, Response, NextFunction } from 'express';
import session from 'express-session';

declare module 'express-session' {
  interface SessionData {
    userId: number;
    ipAddress: string;
  }
}

function getClientIp(req: Request): string {
  const forwarded = req.headers['x-forwarded-for'] as string | undefined;
  return forwarded?.split(',')[0]?.trim() || req.socket.remoteAddress || '';
}

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password } = req.body;
  if (username === 'admin' && password === process.env.ADMIN_PASSWORD) {
    req.session.userId = 1;
    req.session.ipAddress = getClientIp(req);
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
  const currentIp = getClientIp(req);
  if (req.session.ipAddress && req.session.ipAddress !== currentIp) {
    console.warn(`IP mismatch for user ${req.session.userId}: session was ${req.session.ipAddress}, request from ${currentIp}`);
    req.session.destroy(() => {});
    res.status(401).json({ error: 'session invalidated due to IP change' });
    return;
  }
  next();
}
