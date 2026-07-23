// SAFE alternative: Use cookie-session with explicit security options

var express = require('express');
var cookieSession = require('cookie-session');

var app = express();

app.use(cookieSession({
  name: 'session',
  secret: process.env.SESSION_SECRET,
  httpOnly: true,
  secure: true,
  sameSite: 'lax',
  maxAge: 30 * 60 * 1000,
  signed: true,
  overwrite: true
}));

app.get('/profile', function(req, res) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Not authenticated' });
  }
  res.json({ userId: req.session.userId });
});

module.exports = app;
