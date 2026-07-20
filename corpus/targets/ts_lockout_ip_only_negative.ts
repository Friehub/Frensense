// SAFE: Failed attempts tracked per user account; IP tracking is supplementary
const MAX_ATTEMPTS = 5;
const LOCKOUT_MS = 15 * 60 * 1000;

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) return new Response('Invalid credentials', { status: 401 });
  if (user.locked_until > Date.now()) return new Response('Account locked', { status: 429 });
  if (!await bcrypt.compare(password, user.passwordHash)) {
    await db.prepare('UPDATE users SET failed_attempts = failed_attempts + 1 WHERE id = ?').bind(user.id).run();
    if (user.failed_attempts + 1 >= MAX_ATTEMPTS) {
      await db.prepare('UPDATE users SET locked_until = ? WHERE id = ?').bind(Date.now() + LOCKOUT_MS, user.id).run();
    }
    return new Response('Invalid credentials', { status: 401 });
  }
  await db.prepare('UPDATE users SET failed_attempts = 0, locked_until = 0 WHERE id = ?').bind(user.id).run();
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
