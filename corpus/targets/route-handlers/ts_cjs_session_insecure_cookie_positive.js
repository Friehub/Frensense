// [frensense]
// observation: Session cookie is configured without httpOnly, secure, or sameSite flags, making it accessible to client-side JavaScript and sent over unencrypted connections.
// impact: An attacker can steal the session cookie via XSS, or intercept it over insecure HTTP, leading to session hijacking and account takeover.
// improvement: Set httpOnly: true, secure: true, and sameSite: 'strict' (or 'lax') on the session cookie.
// cwe: CWE-384
// cvss: 8.8
// owasp: A07:2021
// severity: High

var express = require('express');
var session = require('express-session');

var app = express();

app.use(session({
  secret: 'my-secret-key',
  resave: true,
  saveUninitialized: true,
  cookie: {
    httpOnly: false,
    secure: false
  }
}));

app.get('/profile', function(req, res) {
  if (!req.session.userId) {
    return res.status(401).json({ error: 'Not authenticated' });
  }
  res.json({ userId: req.session.userId });
});

module.exports = app;
