// SAFE: Trusted device requires re-verification every 24 hours with a signed token.
import { Request, Response, NextFunction } from 'express';
import jwt from 'jsonwebtoken';

const DEVICE_SECRET = process.env.DEVICE_TRUST_SECRET!;
const REVERIFY_INTERVAL = 24 * 60 * 60 * 1000;

export async function verifyMfa(req: Request, res: Response): Promise<void> {
  const { code, trustDevice } = req.body;

  if (code !== '123456') {
    res.status(401).json({ error: 'invalid code' });
    return;
  }

  if (trustDevice) {
    const deviceToken = jwt.sign(
      { userId: req.session.userId, trustedAt: Date.now() },
      DEVICE_SECRET,
      { expiresIn: '30d' },
    );
    res.cookie('device_trust', deviceToken, {
      httpOnly: true,
      secure: true,
      sameSite: 'strict',
      maxAge: 30 * 24 * 60 * 60 * 1000,
    });
  }
  req.session.mfaVerified = true;
  res.json({ ok: true });
}

export function requireMfa(req: Request, res: Response, next: NextFunction): void {
  const deviceToken = req.cookies['device_trust'] as string | undefined;
  if (deviceToken) {
    try {
      const payload = jwt.verify(deviceToken, DEVICE_SECRET) as { userId: number; trustedAt: number };
      if (Date.now() - payload.trustedAt < REVERIFY_INTERVAL) {
        next();
        return;
      }
      res.clearCookie('device_trust');
    } catch {
      res.clearCookie('device_trust');
    }
  }
  if (req.session.mfaVerified) {
    next();
    return;
  }
  res.status(401).json({ error: 'MFA required' });
}
