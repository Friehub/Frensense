// SAFE: Rate limit uses an exponential backoff that increases on each failure regardless of interleaved successes
import { Request, Response } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function loginHandler(req: Request, res: Response): Promise<void> {
  const key = `login:backoff:${req.ip}`;
  const backoff = await redis.get(key);
  if (backoff && parseInt(backoff) > Date.now()) {
    res.status(429).json({ error: 'Try again later' });
    return;
  }
  const success = await authenticateUser(req.body);
  if (!success) {
    const failures = await redis.incr(`login:failures:${req.ip}`);
    const waitMs = Math.min(300000, Math.pow(2, failures) * 1000);
    await redis.set(key, String(Date.now() + waitMs), { EX: 3600 });
    res.status(401).json({ error: 'Invalid credentials' });
  } else {
    await redis.del(`login:failures:${req.ip}`);
    res.json({ token: crypto.randomUUID() });
  }
}

async function authenticateUser(body: any): Promise<boolean> {
  return false;
}
