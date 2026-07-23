// [frensense]
// observation: The rate limit can be bypassed by setting certain HTTP headers (X-Real-IP, CF-Connecting-IP) that override the actual source IP.
// impact: An attacker can set arbitrary headers to impersonate different IP addresses, bypassing IP-based rate limiting entirely.
// improvement: Use only the verified upstream IP from the proxy (e.g., CF-Connecting-IP on Cloudflare) and never trust client-provided headers.

export async function handler(req: Request, env: Env) {
  // VULNERABLE: trusts client-provided IP headers
  const ip = req.headers.get('X-Real-IP') ||
    req.headers.get('CF-Connecting-IP') ||
    req.headers.get('x-forwarded-for')?.split(',')[0] ||
    'unknown';

  const key = `ratelimit:${ip}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 10) {
    return new Response('Too many requests', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 60 });
  await processRequest(req, env);
}
