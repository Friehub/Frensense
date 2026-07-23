// SAFE: Uses per-service signing keys so one service cannot accept another service's token
import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';

const SERVICE_KEYS: Record<string, string> = {
  'reporting-service': process.env.REPORTING_SERVICE_SECRET!,
};

export function createDelegatedToken(userId: string, scopes: string[], targetService: string): string {
  const secret = SERVICE_KEYS[targetService];
  if (!secret) throw new Error('Unknown target service');
  return jwt.sign(
    { sub: userId, scopes, svc: targetService },
    secret,
    { expiresIn: '1h' }
  );
}

export async function handleDelegatedRequest(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  const secret = SERVICE_KEYS['reporting-service'];
  try {
    jwt.verify(token!, secret);
  } catch {
    res.status(401).json({ error: 'Token not valid for this service' });
    return;
  }
  res.json({ ok: true });
}
