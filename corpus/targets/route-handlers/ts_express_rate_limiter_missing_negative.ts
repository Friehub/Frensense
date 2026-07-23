// SAFE: express-rate-limit is applied to the login route, allowing only 5 attempts per 15 minutes per IP.

import express from 'express';
import rateLimit from 'express-rate-limit';

const app = express();
app.use(express.json());

const loginLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 5,
  message: { error: 'Too many login attempts, please try again later' },
  standardHeaders: true,
  legacyHeaders: false,
});

app.post('/api/login', loginLimiter, (req, res) => {
  const { username, password } = req.body;
  if (username === 'admin' && password === 's3cret') {
    res.json({ token: 'valid-session-token' });
  } else {
    res.status(401).json({ error: 'Invalid credentials' });
  }
});
