// SAFE alternative: use auth event logging service
import { authEvents } from './auth-events';

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    await authEvents.logFailure({ email, ip: req.ip, timestamp: new Date() });
    return res.status(401).json({ error: 'Invalid email or password' });
  }

  await authEvents.logSuccess({ userId: user.id, ip: req.ip });
  const token = signToken({ id: user.id });
  res.json({ token });
});
