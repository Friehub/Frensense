// SAFE: Auth check happens at the edge (CDN / gateway) before the request reaches the BFF
import { Request, Response } from 'express';

interface AuthenticatedRequest extends Request {
  userId?: string;
}

export async function proxyApi(req: AuthenticatedRequest, res: Response): Promise<void> {
  if (!req.userId) {
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  const upstream = await fetch('https://api.example.com/data', {
    headers: { Authorization: `Bearer ${await getTokenForUser(req.userId)}` },
  });
  res.json(await upstream.json());
}

async function getTokenForUser(userId: string): Promise<string> {
  return `token-for-${userId}`;
}
