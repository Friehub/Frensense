// [frensense]
// observation: The permission cache uses non-namespaced keys, so users from different tenants (organizations) share the same cache keys. A user in tenant A can get cached permissions from tenant B.
// impact: Cross-tenant permission leakage allows a user from one organization to inherit the elevated permissions of a user with the same user ID in another organization.
// improvement: Namespace all cache keys by tenant ID to isolate permission data between tenants.

import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function loadPermissions(userId: string): Promise<string[]> {
  const cached = await redis.get(`perms:${userId}`);
  if (cached) return JSON.parse(cached);
  const perms = await dbQuery(userId);
  await redis.set(`perms:${userId}`, JSON.stringify(perms), { EX: 600 });
  return perms;
}

async function dbQuery(userId: string): Promise<string[]> {
  return ['read'];
}

export async function authMiddleware(req: Request, res: Response): Promise<void> {
  const perms = await loadPermissions(req.user.userId);
  req.permissions = perms;
}
