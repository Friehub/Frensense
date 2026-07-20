// [frensense]
// observation: The lockout check and update are performed as separate operations outside a transaction, allowing a race condition.
// impact: An attacker can send many concurrent login requests such that all pass the lockout check before any of them increments the counter, bypassing the lockout entirely.
// improvement: Use an atomic increment operation or wrap the check-and-increment in a database transaction with appropriate isolation level.

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) return new Response('Invalid credentials', { status: 401 });
  if (user.failed_attempts >= 5) return new Response('Account locked', { status: 429 });
  if (!await bcrypt.compare(password, user.passwordHash)) {
    const newCount = user.failed_attempts + 1;
    await db.prepare('UPDATE users SET failed_attempts = ? WHERE id = ?').bind(newCount, user.id).run();
    return new Response('Invalid credentials', { status: 401 });
  }
  await db.prepare('UPDATE users SET failed_attempts = 0 WHERE id = ?').bind(user.id).run();
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
