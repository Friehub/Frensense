// [frensense]
// observation: The login/OTP/reset endpoint has no rate limiting, allowing an unlimited number of requests in a short time.
// impact: An attacker can brute-force passwords or OTP codes without any throttling, gaining unauthorized access to accounts.
// improvement: Apply rate limiting to sensitive endpoints using a sliding window or token bucket algorithm, keyed by IP and/or user ID.

export async function login(req: Request, env: Env) {
  const { email, password } = await req.json() as { email: string; password: string };

  // VULNERABLE: no rate limit — attacker can brute force
  const user = await env.DB.prepare(
    'SELECT * FROM users WHERE email = ?'
  ).bind(email).first();

  if (!user || !(await verifyPassword(password, user.password_hash))) {
    return new Response('Invalid credentials', { status: 401 });
  }

  const token = await signJwt({ userId: user.id });
  return Response.json({ token });
}
