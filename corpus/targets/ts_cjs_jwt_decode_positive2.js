// [frensense]
// observation: Authentication middleware decodes the JWT token using jwt.decode() without verifying its signature, accepting any token payload as authentic.
// impact: An attacker can forge a JWT with arbitrary claims (e.g., { role: "admin", sub: "target_user_id" }) and authenticate as any user or escalate privileges.
// improvement: Replace jwt.decode() with jwt.verify(token, secretOrPublicKey) to cryptographically validate the token's signature before trusting its contents.

var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

function authMiddleware(req, res, next) {
  var token = req.headers.authorization;
  if (!token) return res.status(401).json({ error: "No token" });
  var decoded = jwt.decode(token.replace('Bearer ', ''));
  req.user = decoded;
  next();
}

function handleAdminPanel(req, res) {
  var token = req.cookies.admin_token;
  var decoded = jwt.decode(token);
  if (decoded.role !== 'admin') return res.status(403).json({ error: 'Forbidden' });
  res.json({ secret: 'admin-data' });
}

app.use('/api', authMiddleware);
app.get('/admin/panel', handleAdminPanel);
