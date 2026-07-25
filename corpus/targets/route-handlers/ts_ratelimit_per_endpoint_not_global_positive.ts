// [frensense]
// observation: The rate limit is only checked per-endpoint, not globally per user, allowing a user to bypass the limit by rotating through different endpoints.
// impact: An attacker can spread requests across multiple endpoints, each with its own limit, achieving a much higher total request rate than intended.
// improvement: Apply a global per-user rate limit in addition to per-endpoint limits.
// cwe: CWE-770
// cvss: 5.3
// owasp: A04:2021
// severity: Medium

export async function apiHandler(req: Request, env: Env) {
  const auth = await resolveAuth(req);
  if (!auth) return new Response('Unauthorized', { status: 401 });

  const endpoint = new URL(req.url).pathname;

  // VULNERABLE: only per-endpoint limit, no global limit
  const endpointKey = `ratelimit:${endpoint}:${auth.userId}`;
  const current = await env.KV.get(endpointKey);
  const count = current ? parseInt(current) : 0;

  if (count >= 100) {
    return new Response('Rate limit exceeded for this endpoint', { status: 429 });
  }

  await env.KV.put(endpointKey, String(count + 1), { expirationTtl: 60 });
  await handleRequest(req, env);
}
