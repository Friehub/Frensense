// SAFE: Per-request data is stored in the request-scoped context, not module-level variables

interface Env {
  CACHE: KVNamespace;
}

export async function handler(request: Request, env: Env): Promise<Response> {
  const userId = request.headers.get('X-User-Id') || 'anonymous';
  const url = new URL(request.url);

  const cached = await env.CACHE.get(url.pathname, 'json');
  if (cached) {
    return new Response(JSON.stringify(cached));
  }

  const data = await fetchData(userId, url.pathname);
  await env.CACHE.put(url.pathname, JSON.stringify(data), { expirationTtl: 60 });
  return new Response(JSON.stringify(data));
}

async function fetchData(userId: string, path: string): Promise<any> {
  return { userId, path, timestamp: Date.now() };
}
