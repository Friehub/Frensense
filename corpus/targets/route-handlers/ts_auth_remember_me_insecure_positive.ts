// [frensense]
// observation: "Remember me" token stored as a plaintext cookie value (e.g., `remember_me=userId:token`) without encryption or HMAC, enabling session forging.
// impact: Persistent session theft — an attacker who steals the cookie file or sniffs the network can forge remember-me tokens for any user by modifying the userId portion.
// improvement: Store a cryptographically random opaque token in the cookie, mapped server-side to the user session.

import { Request, Response } from 'express';

const rememberMeSecrets = new Map<string, number>();

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const { username, password, rememberMe } = req.body;
  if (username === 'admin' && password === 'secret') {
    if (rememberMe) {
      const token = `${req.sessionID}:admin`;
      res.cookie('remember_me', token, {
        httpOnly: true,
        maxAge: 30 * 24 * 60 * 60 * 1000,
      });
      rememberMeSecrets.set(token, 1);
    }
    res.json({ ok: true });
  } else {
    res.status(401).json({ error: 'invalid credentials' });
  }
}

export async function rememberMeAuth(req: Request, res: Response): Promise<void> {
  const cookie = req.cookies['remember_me'] as string | undefined;
  if (cookie) {
    const parts = cookie.split(':');
    const userId = parseInt(parts[1], 10);
    if (rememberMeSecrets.has(cookie)) {
      req.session.userId = userId;
      res.json({ ok: true, userId });
      return;
    }
  }
  res.status(401).json({ error: 'invalid remember-me token' });
}
