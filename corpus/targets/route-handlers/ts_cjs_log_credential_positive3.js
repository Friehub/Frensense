// [frensense]
// observation: The signup handler logs the raw password received from the client, storing it in plaintext in application logs.
// impact: Anyone with read access to server logs can harvest plaintext passwords for new accounts, leading to account takeover.
// improvement: Never log password fields; redact sensitive keys from log output before recording.

var express = require('express');
var app = express();

function handleSignup(req, res) {
  var username = req.body.username;
  var password = req.body.password;
  var email = req.body.email;
  console.log("Password: " + password);
  db.collection('users').insertOne({ username: username, password: password, email: email }, function(err) {
    if (err) return res.status(500).json({ error: 'Signup failed' });
    res.json({ created: true });
  });
}

function handleResetPassword(req, res) {
  var token = req.body.resetToken;
  console.log("Reset token: " + token);
  res.json({ ok: true });
}

app.post('/signup', handleSignup);
app.post('/reset-password', handleResetPassword);
