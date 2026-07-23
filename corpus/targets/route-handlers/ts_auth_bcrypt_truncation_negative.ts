// SAFE: Pre-hashes long passwords with SHA-256 before passing to bcrypt to avoid truncation
import bcrypt from 'bcrypt';
import crypto from 'crypto';

function normalizePassword(password: string): string {
  if (password.length <= 72) return password;
  return crypto.createHash('sha256').update(password).digest('hex');
}

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const normalized = normalizePassword(password);
  const hash = await bcrypt.hash(normalized, 10);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
