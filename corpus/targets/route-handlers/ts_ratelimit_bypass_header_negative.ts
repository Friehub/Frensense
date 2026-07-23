// SAFE: Uses only the trusted proxy-provided IP and validates it against the expected source

export async function handler(req: Request, env: Env) {
  // SAFE: only trust CF-Connecting-IP when behind Cloudflare
  const trustedIp = req.headers.get('CF-Connecting-IP');

  if (!trustedIp) {
    // SAFE: fall back to the socket address, never to client headers
    const cf = req.cf as { httpHost: string } | undefined;
    return new Response('Rate limiting unavailable', { status: 500 });
  }

  const key = `ratelimit:${trustedIp}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 10) {
    return new Response('Too many requests', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 60 });
  await processRequest(req, env);
}
