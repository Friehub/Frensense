// SAFE alternative: IP-based rate limit alerting
import { RateLimiter } from './rate-limiter';

const loginLimiter = new RateLimiter({ windowMs: 15 * 60 * 1000, max: 10 });

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;

  if (!await loginLimiter.check(req.ip)) {
    await alerting.send({
      severity: 'medium',
      title: 'IP rate-limited at login',
      message: `IP ${req.ip} exceeded rate limit targeting ${email}`,
    });
    return res.status(429).json({ error: 'Too many attempts' });
  }

  const user = await db.findUserByEmail(email);
  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    return res.status(401).json({ error: 'Invalid credentials' });
  }

  res.json({ token: signToken({ id: user.id }) });
});
