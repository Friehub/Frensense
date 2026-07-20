// [frensense]
// observation: The login endpoint returns distinct error messages for invalid usernames vs invalid passwords.
// impact: An attacker can enumerate valid usernames/emails by observing "User not found" vs "Wrong password" messages, then target those accounts for credential stuffing or phishing.
// improvement: Return identical generic error messages for all authentication failures.

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) {
    return new Response(JSON.stringify({ error: 'User not found' }), { status: 401 });
  }
  if (!await bcrypt.compare(password, user.passwordHash)) {
    return new Response(JSON.stringify({ error: 'Wrong password' }), { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
