// SAFE: Account locks after 5 failed attempts for 15 minutes
const MAX_ATTEMPTS = 5;
const LOCKOUT_DURATION = 15 * 60 * 1000;

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) return new Response('Invalid credentials', { status: 401 });
  if (user.locked_until && user.locked_until > Date.now()) {
    return new Response('Account locked. Try again later.', { status: 429 });
  }
  if (!await bcrypt.compare(password, user.passwordHash)) {
    const attempts = user.failed_attempts + 1;
    if (attempts >= MAX_ATTEMPTS) {
      await db.prepare('UPDATE users SET failed_attempts = ?, locked_until = ? WHERE id = ?').bind(attempts, Date.now() + LOCKOUT_DURATION, user.id).run();
      return new Response('Account locked due to too many attempts', { status: 429 });
    }
    await db.prepare('UPDATE users SET failed_attempts = ? WHERE id = ?').bind(attempts, user.id).run();
    return new Response('Invalid credentials', { status: 401 });
  }
  await db.prepare('UPDATE users SET failed_attempts = 0, locked_until = NULL WHERE id = ?').bind(user.id).run();
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
