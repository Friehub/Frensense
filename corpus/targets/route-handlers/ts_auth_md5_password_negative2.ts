// SAFE: Uses Argon2id, the current OWASP-recommended password hashing algorithm
import argon2 from 'argon2';

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await argon2.hash(password, { type: argon2.argon2id, memoryCost: 65536, timeCost: 3 });
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
