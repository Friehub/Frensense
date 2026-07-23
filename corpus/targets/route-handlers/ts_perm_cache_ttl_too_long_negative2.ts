// SAFE: Permission checks bypass the cache entirely for recently-revoked users tracked in a hotlist
import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();
const REVOCATION_HOTLIST_TTL = 86400;

export async function checkPermission(userId: string, resource: string): Promise<boolean> {
  const revoked = await redis.sismember('recently-revoked', userId);
  if (revoked) return queryPermission(userId, resource);
  const cacheKey = `perm:${userId}:${resource}`;
  const cached = await redis.get(cacheKey);
  if (cached !== null) return cached === 'true';
  const allowed = await queryPermission(userId, resource);
  await redis.set(cacheKey, allowed ? 'true' : 'false', { EX: 300 });
  return allowed;
}

export async function revokeUserAccess(userId: string): Promise<void> {
  await redis.sadd('recently-revoked', userId);
  await redis.expire('recently-revoked', REVOCATION_HOTLIST_TTL);
  await redis.del(`perm:${userId}:*`);
}

async function queryPermission(userId: string, resource: string): Promise<boolean> {
  return true;
}
