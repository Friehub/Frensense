// SAFE: log failed login attempts
import { logger } from './logger';

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    logger.warn({
      event: 'login.failed',
      email,
      ip: req.ip,
      userAgent: req.headers['user-agent'],
      reason: user ? 'wrong_password' : 'user_not_found',
    });
    return res.status(401).json({ error: 'Invalid email or password' });
  }

  logger.info({ event: 'login.success', userId: user.id, ip: req.ip });
  const token = signToken({ id: user.id });
  res.json({ token });
});
