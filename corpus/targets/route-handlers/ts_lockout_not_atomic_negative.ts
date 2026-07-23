// SAFE: Failed attempt increment is done atomically via SQL
export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) return new Response('Invalid credentials', { status: 401 });
  if (user.failed_attempts >= 5) return new Response('Account locked', { status: 429 });
  if (!await bcrypt.compare(password, user.passwordHash)) {
    await db.prepare('UPDATE users SET failed_attempts = failed_attempts + 1 WHERE id = ?').bind(user.id).run();
    return new Response('Invalid credentials', { status: 401 });
  }
  await db.prepare('UPDATE users SET failed_attempts = 0, locked_until = NULL WHERE id = ?').bind(user.id).run();
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
