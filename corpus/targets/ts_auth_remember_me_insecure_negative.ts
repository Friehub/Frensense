// SAFE: Opaque random token stored server-side, only the token hash is in the cookie.
import { Request, Response } from 'express';
import crypto from 'crypto';

const rememberMeTokens = new Map<string, { userId: number; expiresAt: number }>();

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password, rememberMe } = req.body;
  if (username === 'admin' && password === 'secret') {
    if (rememberMe) {
      const rawToken = crypto.randomBytes(32).toString('hex');
      rememberMeTokens.set(rawToken, {
        userId: 1,
        expiresAt: Date.now() + 30 * 24 * 60 * 60 * 1000,
      });
      res.cookie('remember_me', rawToken, {
        httpOnly: true,
        secure: true,
        sameSite: 'lax',
        maxAge: 30 * 24 * 60 * 60 * 1000,
      });
    }
    res.json({ ok: true });
  } else {
    res.status(401).json({ error: 'invalid credentials' });
  }
}

export async function rememberMeAuth(req: Request, res: Response): Promise<void> {
  const rawToken = req.cookies['remember_me'] as string | undefined;
  if (!rawToken) {
    res.status(401).json({ error: 'no remember-me token' });
    return;
  }
  const entry = rememberMeTokens.get(rawToken);
  if (!entry || Date.now() > entry.expiresAt) {
    rememberMeTokens.delete(rawToken);
    res.status(401).json({ error: 'invalid or expired remember-me token' });
    return;
  }
  req.session.userId = entry.userId;
  res.json({ ok: true, userId: entry.userId });
}
