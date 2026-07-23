// SAFE: Cookie flags are set via the cookie-parser middleware with explicit secure and httpOnly options as a defense-in-depth layer.

import express from 'express';
import session from 'express-session';
import cookieParser from 'cookie-parser';

const app = express();
app.use(cookieParser());

app.use(session({
  secret: process.env.SESSION_SECRET || 'fallback-secret',
  resave: false,
  saveUninitialized: true,
  cookie: {
    httpOnly: true,
    secure: process.env.NODE_ENV === 'production',
    sameSite: 'lax',
  },
}));

app.get('/api/me', (req, res) => {
  res.json({ user: req.session.user });
});
