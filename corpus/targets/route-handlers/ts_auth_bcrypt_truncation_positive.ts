// [frensense]
// observation: bcrypt silently truncates passwords longer than 72 bytes, but the application accepts arbitrary-length passwords without warning.
// impact: A user with a 73+ character password effectively has a much weaker password (first 72 chars only). Two passwords differing only after byte 72 will authenticate identically.
// improvement: Pre-hash long passwords with SHA-256 before passing to bcrypt, or reject passwords exceeding 72 bytes.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import bcrypt from 'bcrypt';

export async function register(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const hash = await bcrypt.hash(password, 10);
  await db.prepare('INSERT INTO users (email, password_hash) VALUES (?, ?)').bind(email, hash).run();
  return new Response('Created', { status: 201 });
}
