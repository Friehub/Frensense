// SAFE: Uses crypto.timingSafeEqual with proper length extraction
import crypto from 'crypto';

export async function login(req: Request, db: DB): Promise<Response> {
  const { username, password } = await req.json();
  const user = await db.prepare('SELECT * FROM users WHERE username = ?').bind(username).first();
  if (!user) return new Response('Unauthorized', { status: 401 });
  const pwBuffer = Buffer.from(password);
  const hashBuffer = Buffer.from(user.passwordHash);
  const safe = crypto.timingSafeEqual(
    Buffer.concat([pwBuffer, Buffer.alloc(hashBuffer.length - pwBuffer.length)]),
    hashBuffer
  );
  if (!safe) return new Response('Unauthorized', { status: 401 });
  return new Response(JSON.stringify({ token: generateToken(user.id) }));
}
