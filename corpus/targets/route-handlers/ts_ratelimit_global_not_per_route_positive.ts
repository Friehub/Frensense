// [frensense]
// observation: A single global rate limit counter is shared across all API routes, so an attacker can exhaust the limit by hammering a cheap health-check or static-resource endpoint, starving legitimate requests to expensive routes.
// impact: Denial of service against critical endpoints (login, checkout) by flooding a low-cost endpoint that shares the same global rate limit budget.
// improvement: Apply per-route rate limits with separate budgets, and only count expensive endpoints against the global user limit.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();

export async function globalRateLimiter(req: Request, res: Response, next: NextFunction): Promise<void> {
  const key = `ratelimit:global:${req.ip}`;
  const count = await redis.incr(key);
  if (count === 1) await redis.expire(key, 60);
  if (count > 1000) {
    res.status(429).json({ error: 'Global rate limit exceeded' });
    return;
  }
  next();
}

export async function healthCheck(req: Request, res: Response): Promise<void> {
  res.json({ status: 'ok' });
}

export async function checkout(req: Request, res: Response): Promise<void> {
  res.json({ order: 'created' });
}
