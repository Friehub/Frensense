// [frensense]
// observation: The login endpoint has no limit on failed password attempts, allowing unlimited brute-force guesses.
// impact: An attacker can automate millions of password attempts against any user account without being blocked, enabling credential-stuffing and brute-force attacks.
// improvement: Implement account lockout after a threshold of failed attempts (e.g., 5 failed attempts → 15-minute lockout).

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
