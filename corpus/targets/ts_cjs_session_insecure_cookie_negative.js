// SAFE: Session cookie configured with security flags

var express = require('express');
var session = require('express-session');

var app = express();

app.use(session({
  secret: process.env.SESSION_SECRET,
  resave: false,
  saveUninitialized: false,
  cookie: {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    maxAge: 3600000
  }
}));

app.get('/profile', function(req, res) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Not authenticated' });
  }
  res.json({ userId: req.session.userId });
});

module.exports = app;
