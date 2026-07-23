// SAFE: Sliding window rate limiter using a sorted set (sliding log approach)
import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();
const WINDOW_MS = 60000;
const MAX_REQUESTS = 100;

export async function slidingWindowRateLimiter(req: Request, res: Response, next: NextFunction): Promise<void> {
  const key = `rl:sliding:${req.ip}`;
  const now = Date.now();
  const windowStart = now - WINDOW_MS;
  await redis.zRemRangeByScore(key, 0, windowStart);
  const count = await redis.zCard(key);
  if (count >= MAX_REQUESTS) {
    res.status(429).json({ error: 'Rate limit exceeded' });
    return;
  }
  await redis.zAdd(key, { score: now, value: `${now}:${Math.random()}` });
  await redis.expire(key, Math.ceil(WINDOW_MS / 1000));
  next();
}
