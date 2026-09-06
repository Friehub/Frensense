// SAFE: Cache keys are namespaced with tenant ID
import { Redis } from 'ioredis';

const redis = new Redis();

export async function getCachedSettings(req: Request): Promise<Response> {
  const session = getSession(req);
  const { feature } = req.query;
  const cacheKey = `tenant:${session.tenantId}:settings:${feature}`;
  const cached = await redis.get(cacheKey);
  if (cached) return new Response(cached);
  const settings = await db.prepare('SELECT * FROM settings WHERE feature = ? AND tenant_id = ?').bind(feature, session.tenantId).first();
  await redis.set(cacheKey, JSON.stringify(settings), 'EX', 300);
  return new Response(JSON.stringify(settings));
}

export async function getUserProfile(req: Request, db: DB): Promise<Response> {
  const session = getSession(req);
  const cacheKey = `tenant:${session.tenantId}:profile:${req.params.id}`;
  const cached = await redis.get(cacheKey);
  if (cached) return new Response(cached);
  const profile = await db.prepare('SELECT * FROM profiles WHERE user_id = ? AND tenant_id = ?').bind(req.params.id, session.tenantId).first();
  await redis.set(cacheKey, JSON.stringify(profile), 'EX', 300);
  return new Response(JSON.stringify(profile));
}
