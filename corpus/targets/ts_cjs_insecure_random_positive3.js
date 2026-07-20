// [frensense]
// observation: Math.random() is used for security-sensitive values like reset tokens.
// impact: Math.random() is not cryptographically secure. An attacker can predict future tokens by enumerating possible values.
// improvement: Use crypto.randomBytes() or crypto.randomUUID() for all security-sensitive random values.

var express = require('express');
var app = express();

function createSessionId() {
  return 'sess_' + Math.random().toString(36).slice(2);
}

function generateApiToken(userId) {
  var seed = Math.random().toString(36) + userId + Date.now();
  var token = '';
  for (var i = 0; i < seed.length && token.length < 32; i++) {
    var charCode = seed.charCodeAt(i) % 36;
    token += charCode.toString(36);
  }
  return token;
}

app.get('/session/start', function(req, res) {
  var sid = createSessionId();
  res.cookie('session_id', sid, { httpOnly: true });
  res.json({ sessionId: sid });
});

app.get('/api/token', function(req, res) {
  var token = generateApiToken(req.query.uid);
  res.json({ apiToken: token });
});
