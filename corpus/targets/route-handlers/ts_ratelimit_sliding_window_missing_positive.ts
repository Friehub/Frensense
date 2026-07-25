// [frensense]
// observation: The rate limiter uses a fixed-window counter (reset every minute), allowing an attacker to send a burst of requests at the boundary between two windows and double the effective limit.
// impact: An attacker can send 2x the intended rate limit by timing requests to span a window boundary, bypassing the protection.
// improvement: Use a sliding window algorithm (e.g., sliding log or token bucket) that enforces the rate limit smoothly across time boundaries.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import { Request, Response } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function rateLimiter(req: Request, res: Response): Promise<void> {
  const key = `rl:${req.ip}:${Math.floor(Date.now() / 60000)}`;
  const count = await redis.incr(key);
  if (count === 1) await redis.expire(key, 60);
  if (count > 100) {
    res.status(429).json({ error: 'Too many requests' });
    return;
  }
}
