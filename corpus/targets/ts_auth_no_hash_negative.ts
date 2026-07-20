// SAFE: Uses bcrypt to hash passwords before storage and compare on login
import bcrypt from 'bcrypt';

export async function signup(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await bcrypt.hash(password, 12);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}

export async function signin(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user) return new Response('Unauthorized', { status: 401 });
  const match = await bcrypt.compare(password, user.password_hash);
  if (!match) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify({ token: generateToken(user.id) }));
}
