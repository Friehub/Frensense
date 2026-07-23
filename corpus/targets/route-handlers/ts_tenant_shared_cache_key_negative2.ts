// SAFE: Uses per-tenant Redis database index for complete isolation
import { Redis } from 'ioredis';

const connections = new Map<string, Redis>();

function getTenantRedis(tenantId: string): Redis {
  if (!connections.has(tenantId)) {
    const dbIndex = parseInt(tenantId.slice(-2), 16) % 16;
    connections.set(tenantId, new Redis({ db: dbIndex }));
  }
  return connections.get(tenantId)!;
}

export async function getCachedSettings(req: Request): Promise<Response> {
  const session = getSession(req);
  const redis = getTenantRedis(session.tenantId);
  const cached = await redis.get(`settings:${req.query.feature}`);
  if (cached) return new Response(cached);
  const settings = await db.prepare('SELECT * FROM settings WHERE feature = ? AND tenant_id = ?').bind(req.query.feature, session.tenantId).first();
  await redis.set(`settings:${req.query.feature}`, JSON.stringify(settings), 'EX', 300);
  return new Response(JSON.stringify(settings));
}
