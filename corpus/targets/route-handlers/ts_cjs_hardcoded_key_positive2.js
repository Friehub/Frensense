// [frensense]
// observation: API keys and secrets are hardcoded as string literals in source code.
// impact: Anyone with access to the source code repository can extract valid credentials and use them for unauthorized access.
// improvement: Load secrets from environment variables or a secrets manager at runtime.
// cwe: CWE-798
// cvss: 9.8
// owasp: A02:2021
// severity: Critical

var express = require('express');
var app = express();
var jwt = require('jsonwebtoken');

var JWT_SECRET = 'my-super-secret-key-12345';

function issueToken(req, res) {
  var payload = { userId: req.body.userId, role: 'user' };
  var token = jwt.sign(payload, JWT_SECRET, { expiresIn: '1h' });
  res.json({ token: token });
}

function verifyToken(req, res) {
  var token = req.headers.authorization;
  if (!token) return res.status(401).send('No token');
  var decoded = jwt.verify(token.replace('Bearer ', ''), JWT_SECRET);
  res.json({ valid: true, user: decoded });
}

app.post('/auth/login', issueToken);
app.get('/auth/verify', verifyToken);
