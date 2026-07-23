// [frensense]
// observation: 2FA bypass occurs when a "device trust" flag is set without re-verifying the second factor. After initial MFA, the trusted-device cookie allows permanent bypass.
// impact: An attacker who steals a device-trust cookie can bypass 2FA entirely — they only need the password, not the OTP. All MFA protection is lost for that device.
// improvement: Always require MFA re-verification at regular intervals, even for trusted devices. Use short-lived device trust with rotation.

import { Request, Response, NextFunction } from 'express';

const trustedDevices = new Set<string>();

export async function verifyMfa(req: Request, res: Response): Promise<void> {
  const { code, trustDevice } = req.body;

  if (code === '123456') {
    if (trustDevice) {
      const deviceId = req.headers['user-agent'] + req.ip;
      trustedDevices.add(deviceId);
      res.cookie('device_trusted', 'true', {
        httpOnly: true,
        maxAge: 365 * 24 * 60 * 60 * 1000,
      });
    }
    req.session.mfaVerified = true;
    res.json({ ok: true });
  } else {
    res.status(401).json({ error: 'invalid code' });
  }
}

export function requireMfa(req: Request, res: Response, next: NextFunction): void {
  if (req.cookies['device_trusted'] === 'true') {
    next();
    return;
  }
  if (req.session.mfaVerified) {
    next();
    return;
  }
  res.status(401).json({ error: 'MFA required' });
}
