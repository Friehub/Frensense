// SAFE: Separate global and per-route budgets; cheap endpoints only count against their own budget
import { Request, Response, NextFunction } from 'express';
import { createClient } from 'redis';

const redis = createClient();

const CHEAP_ROUTES = new Set(['/health', '/favicon.ico', '/robots.txt']);
const EXPENSIVE_ROUTES = new Set(['/checkout', '/login', '/api/transfer']);

export async function tieredRateLimiter(req: Request, res: Response, next: NextFunction): Promise<void> {
  const route = req.path;
  const ip = req.ip;
  if (CHEAP_ROUTES.has(route)) {
    const key = `ratelimit:cheap:${route}:${ip}`;
    const count = await redis.incr(key);
    if (count === 1) await redis.expire(key, 60);
    if (count > 200) { res.status(429).json({ error: 'Too many requests' }); return; }
    next(); return;
  }
  if (EXPENSIVE_ROUTES.has(route)) {
    const globalKey = `ratelimit:global:${ip}`;
    const routeKey = `ratelimit:expensive:${route}:${ip}`;
    const globalCount = await redis.incr(globalKey);
    const routeCount = await redis.incr(routeKey);
    if (globalCount === 1) await redis.expire(globalKey, 60);
    if (routeCount === 1) await redis.expire(routeKey, 60);
    if (globalCount > 50 || routeCount > 10) {
      res.status(429).json({ error: 'Rate limit exceeded' }); return;
    }
  }
  next();
}
