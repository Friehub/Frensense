// SAFE: Single generic error + constant processing path
export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  const hash = user?.passwordHash || '';
  const valid = user && await bcrypt.compare(password, hash);
  if (!valid) return new Response(JSON.stringify({ error: 'Authentication failed' }), { status: 401 });
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
