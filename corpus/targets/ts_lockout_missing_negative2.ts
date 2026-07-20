// SAFE: Uses Redis-based rate limiting for account lockout (per-user sliding window)
import { Redis } from 'ioredis';

const redis = new Redis();

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const attemptKey = `login:attempts:${email}`;
  const attempts = await redis.incr(attemptKey);
  if (attempts === 1) await redis.expire(attemptKey, 900);
  if (attempts > 5) return new Response('Too many attempts. Try again later.', { status: 429 });
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) return new Response('Invalid credentials', { status: 401 });
  await redis.del(attemptKey);
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
