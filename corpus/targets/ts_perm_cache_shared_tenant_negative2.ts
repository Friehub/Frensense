// SAFE: Uses a composite key with tenant ID as a hash field prefix for Redis hashes
import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function loadPermissions(tenantId: string, userId: string): Promise<string[]> {
  const cacheKey = `tenant:${tenantId}:perms`;
  const cached = await redis.hGet(cacheKey, userId);
  if (cached) return JSON.parse(cached);
  const perms = await dbQuery(tenantId, userId);
  await redis.hSet(cacheKey, userId, JSON.stringify(perms));
  await redis.expire(cacheKey, 600);
  return perms;
}

async function dbQuery(tenantId: string, userId: string): Promise<string[]> {
  return ['read'];
}

export async function authMiddleware(req: Request, res: Response): Promise<void> {
  const perms = await loadPermissions(req.user.tenantId, req.user.userId);
  req.permissions = perms;
}
