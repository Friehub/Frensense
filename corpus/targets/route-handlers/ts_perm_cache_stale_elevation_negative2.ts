// SAFE: Uses a monotonically incrementing permission version number to detect staleness
import { createClient } from 'redis';
import { Request, Response } from 'express';

const redis = createClient();

export async function getUserPermissions(userId: string): Promise<string[]> {
  const version = await redis.get(`permver:${userId}`);
  const cached = await redis.get(`perms:${userId}`);
  if (cached && version) {
    const parsed = JSON.parse(cached);
    if (parsed.version === parseInt(version)) return parsed.perms;
  }
  const perms = await loadPermissionsFromDb(userId);
  const currentVersion = await redis.get(`permver:${userId}`);
  await redis.set(`perms:${userId}`, JSON.stringify({ perms, version: parseInt(currentVersion || '0') }), { EX: 3600 });
  return perms;
}

async function loadPermissionsFromDb(userId: string): Promise<string[]> {
  return ['read'];
}
