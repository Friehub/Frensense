// SAFE: Uses bcrypt with appropriate cost factor
import bcrypt from 'bcrypt';

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await bcrypt.hash(password, 12);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
