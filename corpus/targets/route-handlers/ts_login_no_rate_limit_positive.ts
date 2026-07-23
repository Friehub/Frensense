// [frensense]
// observation: The login endpoint has no rate limiting, allowing unlimited authentication requests from any client.
// impact: An attacker can automate credential stuffing attacks, trying thousands of username/password pairs per second against the application without being blocked.
// improvement: Apply rate limiting to the login endpoint (e.g., 10 requests per minute per IP/account) using middleware like express-rate-limit or a Redis-based sliding window.

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
