// [frensense]
// observation: Cache keys (Redis, Memcached, in-memory) do not include a tenant namespace, so data from one tenant can be served to another.
// impact: A user from Tenant A might see cached data belonging to Tenant B if the data was cached under a non-namespaced key, violating tenant isolation.
// improvement: Prefix all cache keys with the tenant ID to ensure data isolation between tenants.
// cwe: CWE-200
// cvss: 6.5
// owasp: A01:2021
// severity: Medium

import { Redis } from 'ioredis';

const redis = new Redis();

export async function getCachedSettings(req: Request): Promise<Response> {
  const { feature } = req.query;
  const cached = await redis.get(`settings:${feature}`);
  if (cached) return new Response(cached);
  const settings = await db.prepare('SELECT * FROM settings WHERE feature = ?').bind(feature).first();
  await redis.set(`settings:${feature}`, JSON.stringify(settings));
  return new Response(JSON.stringify(settings));
}

export async function getUserProfile(req: Request, db: DB): Promise<Response> {
  const userId = req.params.id;
  const cached = await redis.get(`profile:${userId}`);
  if (cached) return new Response(cached);
  const profile = await db.prepare('SELECT * FROM profiles WHERE user_id = ?').bind(userId).first();
  await redis.set(`profile:${userId}`, JSON.stringify(profile));
  return new Response(JSON.stringify(profile));
}
