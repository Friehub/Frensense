// SAFE: Introduces a constant artificial delay to mask timing differences
export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  const [match] = await Promise.all([
    user ? bcrypt.compare(password, user.passwordHash) : Promise.resolve(false),
    new Promise(resolve => setTimeout(resolve, 200))
  ]);
  if (!user || !match) return new Response('Invalid credentials', { status: 401 });
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
