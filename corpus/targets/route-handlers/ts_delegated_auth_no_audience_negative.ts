// SAFE: Delegated token includes an audience claim tied to the specific service
import { Request, Response } from 'express';
import jwt from 'jsonwebtoken';

export function createDelegatedToken(userId: string, scopes: string[]): string {
  return jwt.sign(
    { sub: userId, scopes, aud: 'reporting-service' },
    process.env.DELEGATION_SECRET!,
    { expiresIn: '1h' }
  );
}

export async function handleDelegatedRequest(req: Request, res: Response): Promise<void> {
  const token = req.headers.authorization?.split(' ')[1];
  const decoded = jwt.verify(token!, process.env.DELEGATION_SECRET!, {
    audience: 'reporting-service',
  }) as any;
  res.json({ user: decoded.sub, scopes: decoded.scopes });
}
