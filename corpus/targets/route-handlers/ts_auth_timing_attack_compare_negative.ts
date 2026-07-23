// SAFE: Uses bcrypt.compare() which performs constant-time comparison
import bcrypt from 'bcrypt';

export async function login(req: Request, db: DB): Promise<Response> {
  const { username, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user) return new Response('Unauthorized', { status: 401 });
  const match = await bcrypt.compare(password, user.passwordHash);
  if (!match) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify({ token: generateToken(user.id) }));
}
