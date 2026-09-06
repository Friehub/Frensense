// SAFE: Permission cache TTL is short (5 minutes), minimizing the window of stale access
import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function checkPermission(userId: string, resource: string): Promise<boolean> {
  const cacheKey = `perm:${userId}:${resource}`;
  const cached = await redis.get(cacheKey);
  if (cached !== null) return cached === 'true';
  const allowed = await queryPermission(userId, resource);
  await redis.set(cacheKey, allowed ? 'true' : 'false', { EX: 300 });
  return allowed;
}

async function queryPermission(userId: string, resource: string): Promise<boolean> {
  return false;
}

export async function apiHandler(req: Request, res: Response): Promise<void> {
  const allowed = await checkPermission(req.user.userId, req.path);
  if (!allowed) { res.status(403).json({ error: 'Forbidden' }); return; }
  res.json({ data: 'sensitive' });
}
