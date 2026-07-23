// [frensense]
// observation: User passwords are hashed using the MD5 algorithm, which is cryptographically broken and vulnerable to collision and preimage attacks.
// impact: An attacker who gains access to the password database can reverse or rainbow-table MD5 hashes to recover plaintext passwords.
// improvement: Use a strong, adaptive hashing algorithm like bcrypt, scrypt, or Argon2.

import crypto from 'crypto';

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = crypto.createHash('md5').update(password).digest('hex');
  await db.prepare('INSERT INTO users (email, password_md5) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
