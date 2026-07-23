// [frensense]
// observation: Failed login attempts are tracked by IP address only, not by user account.
// impact: An attacker can rotate IP addresses (via VPN, proxies, or botnets) to bypass the IP-based lockout and continue brute-forcing a single user account indefinitely.
// improvement: Track failed attempts per user account, not per IP. Use IP as a supplementary signal only.

const ipAttempts = new Map<string, number>();

export async function login(req: Request, db: DB): Promise<Response> {
  const ip = req.ip;
  const attempts = ipAttempts.get(ip) || 0;
  if (attempts > 5) return new Response('Too many attempts', { status: 429 });
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    ipAttempts.set(ip, attempts + 1);
    return new Response('Invalid credentials', { status: 401 });
  }
  ipAttempts.delete(ip);
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
