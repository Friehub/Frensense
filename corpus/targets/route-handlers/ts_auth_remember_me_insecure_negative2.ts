// SAFE: Remember-me token is a signed JWT stored in the cookie, validated server-side.
import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';

const REMEMBER_ME_SECRET = process.env.REMEMBER_ME_SECRET!;

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password, rememberMe } = req.body;
  if (username === 'admin' && password === 'secret') {
    if (rememberMe) {
      const token = jwt.sign(
        { userId: 1, purpose: 'remember-me' },
        REMEMBER_ME_SECRET,
        { expiresIn: '30d' },
      );
      res.cookie('remember_me', token, {
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
  const token = req.cookies['remember_me'] as string | undefined;
  if (!token) {
    res.status(401).json({ error: 'no remember-me token' });
    return;
  }
  try {
    const payload = jwt.verify(token, REMEMBER_ME_SECRET) as { userId: number };
    req.session.userId = payload.userId;
    res.json({ ok: true, userId: payload.userId });
  } catch {
    res.status(401).json({ error: 'invalid or expired remember-me token' });
  }
}
