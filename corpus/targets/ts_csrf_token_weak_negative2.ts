// SAFE alternative: double-submit cookie pattern with random token
import { randomUUID } from 'node:crypto';

// CSRF token set as cookie + sent in header; server compares them
app.use((req, res, next) => {
  if (!req.cookies['csrf-token']) {
    const token = randomUUID();
    res.cookie('csrf-token', token, { httpOnly: false, sameSite: 'strict', secure: true });
  }
  next();
});

app.post('/api/transfer', (req, res) => {
  const headerToken = req.headers['x-csrf-token'];
  const cookieToken = req.cookies['csrf-token'];
  if (!headerToken || headerToken !== cookieToken) {
    return res.status(403).json({ error: 'Invalid CSRF token' });
  }
  // process transfer
});
