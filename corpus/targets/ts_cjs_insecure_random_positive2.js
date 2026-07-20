// [frensense]
// observation: Math.random() is used for security-sensitive values like reset tokens.
// impact: Math.random() is not cryptographically secure. An attacker can predict future tokens by enumerating possible values.
// improvement: Use crypto.randomBytes() or crypto.randomUUID() for all security-sensitive random values.

var express = require('express');
var app = express();

function issuePasswordResetToken(email) {
  var prefix = 'pwd_reset_';
  var rnd = Math.random().toString(36).substring(2, 10);
  var ts = Date.now().toString(36);
  return prefix + ts + '_' + rnd;
}

function generateEmailConfirmCode(userId) {
  var code = '';
  for (var i = 0; i < 8; i++) {
    code += Math.floor(Math.random() * 10).toString();
  }
  return userId + '_' + code;
}

app.post('/auth/forgot', function(req, res) {
  var token = issuePasswordResetToken(req.body.email);
  res.json({ resetToken: token });
});

app.post('/auth/confirm', function(req, res) {
  var code = generateEmailConfirmCode(req.body.userId);
  res.json({ confirmationCode: code });
});
