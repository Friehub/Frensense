// [frensense]
// observation: The login endpoint checks user existence with a fast DB lookup before performing the slow bcrypt comparison, creating a measurable timing difference between valid and invalid usernames.
// impact: An attacker can determine which email addresses are registered by measuring response times — invalid emails return quickly (no DB row), while valid emails take longer (bcrypt comparison).
// improvement: Always perform the same sequence of operations regardless of whether the user exists. Hash a dummy value for non-existent users to normalize timing.

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) {
    return new Response('Invalid credentials', { status: 401 });
  }
  if (!await bcrypt.compare(password, user.passwordHash)) {
    return new Response('Invalid credentials', { status: 401 });
  }
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
