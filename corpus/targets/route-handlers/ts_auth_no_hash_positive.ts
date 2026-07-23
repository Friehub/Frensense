// [frensense]
// observation: User passwords are stored as plaintext in the database without any hashing or encryption.
// impact: A database breach exposes all user passwords in cleartext, enabling account takeover on this service and any other service where the user reuses credentials.
// improvement: Always hash passwords using a strong adaptive algorithm like bcrypt before storing.

export async function signup(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  await db.prepare('INSERT INTO users (email, password) VALUES (?, ?)').bind(email, password).run();
  return new Response('Created', { status: 201 });
}

export async function signin(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ? AND password = ?').bind(email, password).first();
  if (!user) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify({ token: generateToken(user.id) }));
}
