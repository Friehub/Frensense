// SAFE: Rate-limited login endpoint using KV-based sliding window

export async function login(req: Request, env: Env) {
  const { email, password } = await req.json() as { email: string; password: string };
  const ip = req.headers.get('CF-Connecting-IP') || 'unknown';

  // SAFE: rate limit check
  const key = `ratelimit:login:${ip}`;
  const current = await env.KV.get(key);
  const count = current ? parseInt(current) : 0;

  if (count >= 5) {
    return new Response('Too many attempts. Try again later.', { status: 429 });
  }

  await env.KV.put(key, String(count + 1), { expirationTtl: 300 });

  const user = await env.DB.prepare(
    'SELECT * FROM users WHERE email = ?'
  ).bind(email).first();

  if (!user || !(await verifyPassword(password, user.password_hash))) {
    return new Response('Invalid credentials', { status: 401 });
  }

  const token = await signJwt({ userId: user.id });
  return Response.json({ token });
}
