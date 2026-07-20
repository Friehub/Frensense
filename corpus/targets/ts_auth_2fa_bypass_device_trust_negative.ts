// SAFE: Device trust has a 7-day TTL after which MFA is re-required.
import { Request, Response, NextFunction } from 'express';

const trustedDevices = new Map<string, number>();
const TRUST_TTL_MS = 7 * 24 * 60 * 60 * 1000;

export async function verifyMfa(req: Request, res: Response): Promise<void> {
  const { code, trustDevice } = req.body;

  if (code !== '123456') {
    res.status(401).json({ error: 'invalid code' });
    return;
  }

  if (trustDevice) {
    const deviceId = req.headers['user-agent'] + req.ip;
    const expiresAt = Date.now() + TRUST_TTL_MS;
    trustedDevices.set(deviceId, expiresAt);
    res.cookie('device_trusted', 'true', {
      httpOnly: true,
      secure: true,
      sameSite: 'strict',
      maxAge: TRUST_TTL_MS,
    });
  }
  req.session.mfaVerified = true;
  res.json({ ok: true });
}

export function requireMfa(req: Request, res: Response, next: NextFunction): void {
  if (req.cookies['device_trusted'] === 'true') {
    const deviceId = req.headers['user-agent'] + req.ip;
    const expiresAt = trustedDevices.get(deviceId);
    if (expiresAt && Date.now() < expiresAt) {
      next();
      return;
    }
    res.clearCookie('device_trusted');
  }
  if (req.session.mfaVerified) {
    next();
    return;
  }
  res.status(401).json({ error: 'MFA required' });
}
