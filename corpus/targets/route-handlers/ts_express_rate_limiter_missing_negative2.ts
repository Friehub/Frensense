// SAFE: Rate limiting is applied globally to all routes using slowDown, with stricter limits on the login endpoint.

import express from 'express';
import rateLimit from 'express-rate-limit';
import slowDown from 'express-slow-down';

const app = express();
app.use(express.json());

const globalSlowDown = slowDown({
  windowMs: 15 * 60 * 1000,
  delayAfter: 100,
  delayMs: 500,
});

const loginLimiter = rateLimit({
  windowMs: 15 * 60 * 1000,
  max: 5,
  message: { error: 'Too many login attempts, please try again later' },
});

app.use(globalSlowDown);
app.post('/api/login', loginLimiter, (req, res) => {
  const { username, password } = req.body;
  if (username === 'admin' && password === 's3cret') {
    res.json({ token: 'valid-session-token' });
  } else {
    res.status(401).json({ error: 'Invalid credentials' });
  }
});
