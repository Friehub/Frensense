// SAFE alternative: use csurf or csrf-csrf middleware
import { doubleCsrf } from 'csrf-csrf';

const { generateToken, doubleCsrfProtection } = doubleCsrf({
  getSecret: () => process.env.CSRF_SECRET,
  cookieName: 'csrf-token',
  cookieOptions: { httpOnly: true, sameSite: 'strict', secure: true },
});

app.use(doubleCsrfProtection);

app.get('/api/csrf-token', (req, res) => {
  res.json({ token: generateToken(req, res) });
});
