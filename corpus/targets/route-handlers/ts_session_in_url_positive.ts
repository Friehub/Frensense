// [frensense]
// observation: Authentication tokens or session identifiers are passed as URL query parameters.
// impact: Tokens leak via browser history, server logs, referrer headers, and are visible in the address bar. Any third-party script or analytics tool can read them.
// improvement: Transmit session tokens exclusively through secure, HttpOnly cookies or Authorization headers.
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

export async function authenticate(req: Request): Promise<Response> {
  const token = req.query.token as string;
  if (!token) return new Response('Unauthorized', { status: 401 });
  const user = await verifyToken(token);
  if (!user) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify(user));
}

function buildLoginUrl(redirect: string): string {
  return `/login?session=${generateSessionId()}&redirect=${encodeURIComponent(redirect)}`;
}
