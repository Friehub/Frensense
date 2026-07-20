// [frensense]
// observation: express-session is configured without httpOnly, secure, or sameSite cookie options, so the session cookie can be read by client-side JavaScript, sent over unencrypted HTTP, and is vulnerable to CSRF.
// impact: An attacker can steal the session cookie via XSS (missing httpOnly), intercept it over plain HTTP (missing secure), or forge cross-site requests (missing sameSite), leading to session hijacking.
// improvement: Set cookie.httpOnly: true, cookie.secure: true, and cookie.sameSite: 'strict' in the session configuration.

import express from 'express';
import session from 'express-session';

const app = express();

app.use(session({
  secret: process.env.SESSION_SECRET || 'fallback-secret',
  resave: false,
  saveUninitialized: true,
}));

app.get('/api/me', (req, res) => {
  res.json({ user: req.session.user });
});
