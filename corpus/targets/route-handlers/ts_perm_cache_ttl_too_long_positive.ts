// [frensense]
// observation: The permission cache TTL is set to 24 hours (86400 seconds), so a user whose access is revoked still benefits from cached elevated permissions for an entire day.
// impact: A terminated employee or a user whose subscription has been cancelled can continue accessing protected resources for up to 24 hours after revocation.
// improvement: Use a short TTL (5-15 minutes) for permission caches and actively invalidate the cache on permission changes.

import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function checkPermission(userId: string, resource: string): Promise<boolean> {
  const cacheKey = `perm:${userId}:${resource}`;
  const cached = await redis.get(cacheKey);
  if (cached !== null) return cached === 'true';
  const allowed = await queryPermission(userId, resource);
  await redis.set(cacheKey, allowed ? 'true' : 'false', { EX: 86400 });
  return allowed;
}

async function queryPermission(userId: string, resource: string): Promise<boolean> {
  return true;
}

export async function apiHandler(req: Request, res: Response): Promise<void> {
  const allowed = await checkPermission(req.user.userId, req.path);
  if (!allowed) { res.status(403).json({ error: 'Forbidden' }); return; }
  res.json({ data: 'sensitive' });
}
