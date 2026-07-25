// [frensense]
// observation: The rate limit is keyed only by IP address, which can be bypassed by spoofing X-Forwarded-For headers or rotating through proxies.
// impact: An attacker can bypass rate limiting by sending requests through multiple IP addresses or spoofing the X-Forwarded-For header.
// improvement: Key rate limits by a combination of IP, user ID, and session, and validate proxy headers against a trusted proxy list.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

export async function handler(req: Request, env: Env) {
  const ip = req.headers.get('x-forwarded-for')?.split(',')[0]?.trim() || 'unknown';

  // VULNERABLE: rate limit by IP only — easily bypassed with header spoofing
  const key = `ratelimit:${ip}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 10) {
    return new Response('Too many requests', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 60 });
  await processRequest(req, env);
}
