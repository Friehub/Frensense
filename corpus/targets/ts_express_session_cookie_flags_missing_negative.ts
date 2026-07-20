// SAFE: Session cookie is configured with httpOnly, secure, and sameSite to protect against XSS, MITM, and CSRF.

import express from 'express';
import session from 'express-session';

const app = express();

app.use(session({
  secret: process.env.SESSION_SECRET || 'fallback-secret',
  resave: false,
  saveUninitialized: true,
  cookie: {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
  },
}));

app.get('/api/me', (req, res) => {
  res.json({ user: req.session.user });
});
