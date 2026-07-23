// SAFE: Token bucket algorithm for smooth rate limiting
import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();
const BUCKET_CAPACITY = 100;
const REFILL_RATE = 10;
const REFILL_INTERVAL = 1;

export async function tokenBucketRateLimiter(req: Request, res: Response, next: NextFunction): Promise<void> {
  const key = `rl:bucket:${req.ip}`;
  const now = Date.now() / 1000;
  const bucket = await redis.hGetAll(key);
  let tokens = bucket.tokens ? parseFloat(bucket.tokens) : BUCKET_CAPACITY;
  let lastRefill = bucket.lastRefill ? parseFloat(bucket.lastRefill) : now;
  const elapsed = now - lastRefill;
  tokens = Math.min(BUCKET_CAPACITY, tokens + elapsed * REFILL_RATE);
  if (tokens < 1) {
    res.status(429).json({ error: 'Rate limit exceeded' });
    return;
  }
  await redis.hSet(key, { tokens: String(tokens - 1), lastRefill: String(now) });
  await redis.expire(key, 60);
  next();
}
