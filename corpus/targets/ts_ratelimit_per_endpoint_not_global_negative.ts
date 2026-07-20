// SAFE: Applies both a global per-user limit and a per-endpoint limit

export async function apiHandler(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  const endpoint = new URL(req.url).pathname;

  // SAFE: global rate limit across all endpoints
  const globalKey = `ratelimit:global:${auth.userId}`;
  const globalCurrent = await env.KV.get(globalKey);
  const globalCount = globalCurrent ? parseInt(globalCurrent) : 0;

  if (globalCount >= 500) {
    return new Response('Global rate limit exceeded', { status: 429 });
  }

  await env.KV.put(globalKey, String(globalCount + 1), { expirationTtl: 60 });

  // SAFE: per-endpoint rate limit
  const endpointKey = `ratelimit:${endpoint}:${auth.userId}`;
  const current = await env.KV.get(endpointKey);
  const count = current ? parseInt(current) : 0;

  if (count >= 100) {
    return new Response('Rate limit exceeded for this endpoint', { status: 429 });
  }

  await env.KV.put(endpointKey, String(count + 1), { expirationTtl: 60 });
  await handleRequest(req, env);
}
