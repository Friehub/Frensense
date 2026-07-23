// [frensense]
// observation: Password comparison uses (===) which is not constant-time, leaking character-by-character timing information.
// impact: An attacker can determine the correct password through network timing measurements, drastically reducing brute-force attempts.
// improvement: Use crypto.timingSafeEqual() or bcrypt.compare() for constant-time comparison.

import crypto from 'crypto';

export async function login(req: Request, db: DB): Promise<Response> {
  const { username, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user) return new Response('Unauthorized', { status: 401 });
  if (user.password !== password) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify({ token: generateToken(user.id) }));
}
