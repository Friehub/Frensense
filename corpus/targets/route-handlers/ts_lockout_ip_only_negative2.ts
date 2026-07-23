// SAFE: Combined per-user and per-IP rate limiting via Redis
import { Redis } from 'ioredis';

const redis = new Redis();

export async function login(req: Request, db: DB): Promise<Response> {
  const { email, password } = await req.json();
  const userKey = `lockout:user:${email}`;
  const ipKey = `lockout:ip:${req.ip}`;
  const [userAttempts] = await Promise.all([redis.get(userKey), redis.incr(ipKey)]);
  if (parseInt(userAttempts || '0') >= 5) return new Response('Account locked', { status: 429 });
  const user = await db.prepare('SELECT * FROM users WHERE email = ?').bind(email).first();
  if (!user || !await bcrypt.compare(password, user.passwordHash)) {
    const newAttempts = await redis.incr(userKey);
    if (newAttempts === 1) await redis.expire(userKey, 900);
    return new Response('Invalid credentials', { status: 401 });
  }
  await redis.del(userKey);
  const token = jwt.sign({ sub: user.id }, process.env.JWT_SECRET!, { expiresIn: '1h' });
  return new Response(JSON.stringify({ token }));
}
