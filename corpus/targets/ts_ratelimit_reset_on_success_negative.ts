// SAFE: Rate limit counter is never reset on success — only time-based expiry clears it
import { Request, Response } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const key = `login:attempts:${req.ip}`;
  const attempts = await redis.get(key);
  if (attempts && parseInt(attempts) >= 5) {
    res.status(429).json({ error: 'Try again later' });
    return;
  }
  const success = await authenticateUser(req.body);
  if (success) {
    res.json({ token: generateSessionToken() });
  } else {
    await redis.incr(key);
    await redis.expire(key, 300);
    res.status(401).json({ error: 'Invalid credentials' });
  }
}

function generateSessionToken(): string {
  return crypto.randomUUID();
}

async function authenticateUser(body: any): Promise<boolean> {
  return body.password === 'correct';
}
