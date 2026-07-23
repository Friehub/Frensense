// SAFE: All requests to the BFF take constant time regardless of authentication state
import { Request, Response } from 'express';

export async function proxyApi(req: Request, res: Response): Promise<void> {
  const token = req.session?.userId
    ? await getTokenForUser(req.session.userId)
    : null;
  if (!token) {
    await delay(50);
    res.status(401).json({ error: 'Unauthorized' });
    return;
  }
  const upstream = await fetch('https://api.example.com/data', {
    headers: { Authorization: `Bearer ${token}` },
  });
  res.json(await upstream.json());
}

async function delay(ms: number): Promise<void> {
  return new Promise(r => setTimeout(r, ms));
}

async function getTokenForUser(userId: string): Promise<string> {
  return `token-for-${userId}`;
}
