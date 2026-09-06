// SAFE: Cache keys are namespaced by tenant ID to isolate permissions between organizations
import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function loadPermissions(tenantId: string, userId: string): Promise<string[]> {
  const cacheKey = `tenant:${tenantId}:perms:${userId}`;
  const cached = await redis.get(cacheKey);
  if (cached) return JSON.parse(cached);
  const perms = await dbQuery(tenantId, userId);
  await redis.set(cacheKey, JSON.stringify(perms), { EX: 600 });
  return perms;
}

async function dbQuery(tenantId: string, userId: string): Promise<string[]> {
  return ['read'];
}

export async function authMiddleware(req: Request, res: Response): Promise<void> {
  const perms = await loadPermissions(req.user.tenantId, req.user.userId);
  req.permissions = perms;
}
