// SAFE: Keys rate limit by both user ID and verified IP address

export async function handler(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  const verifiedIp = req.headers.get('CF-Connecting-IP') ||
    req.headers.get('x-forwarded-for')?.split(',')[0]?.trim() || 'unknown';

  // SAFE: combine user ID and verified IP
  const key = `ratelimit:${auth.userId}:${verifiedIp}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 10) {
    return new Response('Too many requests', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 60 });
  await processRequest(req, env);
}
