// SAFE: Session cookie using express-session middleware with secure defaults

var express = require('express');
var session = require('express-session');

var app = express();

app.use(session({
  name: 'sessionId',
  secret: process.env.SESSION_SECRET || 'fallback-secret-change-me',
  resave: false,
  saveUninitialized: false,
  cookie: {
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    maxAge: 86400000
  }
}));

function dashboard(req, res) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Not authenticated' });
  }
  res.json({ user: req.session.userId, role: req.session.role });
}

function setPrefs(req, res) {
  req.session.theme = req.body.theme || 'light';
  res.json({ saved: true });
}

app.get('/dashboard', dashboard);
app.post('/preferences', setPrefs);
