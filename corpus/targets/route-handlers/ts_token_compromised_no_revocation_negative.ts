// SAFE: Every token validation checks a revocation list in Redis
import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function requireValidToken(req: Request, res: Response, next: NextFunction): Promise<void> {
  const authHeader = req.headers.authorization;
  if (!authHeader) { res.status(401).json({ error: 'Missing token' }); return; }
  const token = authHeader.split(' ')[1];
  const isRevoked = await redis.sismember('revoked_tokens', token);
  if (isRevoked) { res.status(401).json({ error: 'Token revoked' }); return; }
  try {
    req.user = jwt.verify(token, process.env.JWT_SECRET!);
    next();
  } catch {
    res.status(401).json({ error: 'Invalid token' });
  }
}
