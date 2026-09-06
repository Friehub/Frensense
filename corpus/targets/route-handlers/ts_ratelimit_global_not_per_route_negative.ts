// SAFE: Per-route rate limit budgets isolate critical endpoints from cheap ones
import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();
const ROUTE_LIMITS: Record<string, number> = {
  '/health': 100,
  '/checkout': 20,
  '/login': 10,
};

export async function perRouteRateLimiter(req: Request, res: Response, next: NextFunction): Promise<void> {
  const route = req.path;
  const limit = ROUTE_LIMITS[route];
  if (!limit) { next(); return; }
  const key = `ratelimit:route:${route}:${req.ip}`;
  const count = await redis.incr(key);
  if (count === 1) await redis.expire(key, 60);
  if (count > limit) {
    res.status(429).json({ error: `Rate limit for ${route} exceeded` });
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
