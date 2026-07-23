// [frensense]
// observation: User permissions are cached in Redis after login but never invalidated when the user's role is changed in the database. The stale cache continues to serve old (elevated) permissions.
// impact: A user demoted from admin to viewer retains admin access for the duration of the cache TTL (potentially hours), enabling unauthorized operations.
// improvement: Invalidate the permission cache whenever the user's role or permissions change in the database.

import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function getUserPermissions(userId: string): Promise<string[]> {
  const cached = await redis.get(`perms:${userId}`);
  if (cached) return JSON.parse(cached);
  const perms = await loadPermissionsFromDb(userId);
  await redis.set(`perms:${userId}`, JSON.stringify(perms), { EX: 3600 });
  return perms;
}

async function loadPermissionsFromDb(userId: string): Promise<string[]> {
  return ['read', 'write', 'delete'];
}

export async function checkAccess(req: Request, res: Response, required: string): Promise<void> {
  const perms = await getUserPermissions(req.user.userId);
  if (!perms.includes(required)) {
    res.status(403).json({ error: 'Forbidden' });
    return;
  }
  res.json({ ok: true });
}
