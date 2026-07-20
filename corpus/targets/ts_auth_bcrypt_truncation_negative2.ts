// SAFE: Rejects passwords longer than 72 bytes with a clear error message
import bcrypt from 'bcrypt';

const MAX_PASSWORD_BYTES = 72;

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const pwBytes = Buffer.byteLength(password, 'utf-8');
  if (pwBytes > MAX_PASSWORD_BYTES) {
    return new Response(JSON.stringify({ error: 'Password too long' }), { status: 400 });
  }
  const hash = await bcrypt.hash(password, 10);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
