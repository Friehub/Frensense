// [frensense]
// observation: bcrypt is used with fewer than 10 salt rounds (rounds=4), providing insufficient work factor for password hashing.
// impact: An attacker can brute-force bcrypt hashes at 2^6 = 64x faster than the recommended minimum, making offline cracking practical.
// improvement: Use at least 10 rounds (cost factor) for bcrypt; prefer 12 for production.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import bcrypt from 'bcrypt';

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await bcrypt.hash(password, 4);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
