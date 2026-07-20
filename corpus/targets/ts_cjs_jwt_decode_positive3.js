// [frensense]
// observation: The Authorization header's Bearer token is extracted by splitting on space and decoded with jwt.decode() without any signature verification.
// impact: Since jwt.decode() does not validate the HMAC or RSA signature, an attacker can craft a token with any payload and bypass all authentication checks.
// improvement: Always use jwt.verify() with the correct secret or public key; jwt.decode() should only be used for debugging or after verification.

var jwt = require('jsonwebtoken');
var express = require('express');
var app = express();

function handleDashboard(req, res) {
  var authHeader = req.headers.authorization;
  if (!authHeader) return res.status(401).json({ error: "Unauthorized" });
  var token = authHeader.split(' ')[1];
  var payload = jwt.decode(token);
  if (!payload) return res.status(401).json({ error: "Invalid token" });
  db.collection('users').findOne({ id: payload.sub }, function(err, user) {
    if (err) return res.status(500).send(err);
    res.json({ dashboard: user.dashboard });
  });
}

function handleWebSocketAuth(req, res) {
  var token = req.query.token;
  var data = jwt.decode(token);
  req.userId = data.userId;
  res.json({ authorized: true });
}

app.get('/dashboard', handleDashboard);
app.get('/ws-auth', handleWebSocketAuth);
