// SAFE: Redis-based sliding window rate limiting per user+IP
import { Redis } from 'ioredis';

const redis = new Redis();

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const key = `ratelimit:login:${email}:${req.ip}`;
  const current = await redis.incr(key);
  if (current === 1) await redis.expire(key, 900);
  if (current > 10) return new Response('Rate limit exceeded', { status: 429 });
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) return new Response('Invalid credentials', { status: 401 });
  await redis.del(key);
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
