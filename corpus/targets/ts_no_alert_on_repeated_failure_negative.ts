// SAFE: alert when repeated failures detected
import { alerting } from './alerting';

const MAX_FAILURES = 5;
const WINDOW_MINUTES = 15;

app.post('/api/login', async (req, res) => {
  const { email, password } = req.body;
  const user = await db.findUserByEmail(email);

  if (!user || !(await bcrypt.compare(password, user.passwordHash))) {
    await db.query(
      'INSERT INTO login_attempts (email, ip, success, created_at) VALUES ($1, $2, false, NOW())',
      [email, req.ip]
    );

    // SAFE: check for repeated failures
    const recent = await db.queryOne(
      `SELECT COUNT(*) as count FROM login_attempts
       WHERE email = $1 AND success = false AND created_at > NOW() - INTERVAL '${WINDOW_MINUTES} minutes'`,
      [email]
    );

    if (recent.count >= MAX_FAILURES) {
      await alerting.send({
        severity: 'high',
        title: 'Repeated login failures detected',
        message: `User ${email}: ${recent.count} failures in ${WINDOW_MINUTES} minutes`,
        ip: req.ip,
      });
    }

    return res.status(401).json({ error: 'Invalid credentials' });
  }

  res.json({ token: signToken({ id: user.id }) });
});
