// [frensense]
// observation: A "remember me" persistent cookie is set without httpOnly, secure, or sameSite flags, making it accessible to JavaScript and transmitted over HTTP.
// impact: An attacker who achieves XSS can steal the cookie via document.cookie, or intercept it over unencrypted connections, enabling persistent session hijacking.
// improvement: Set httpOnly, secure, and sameSite flags on all cookies, especially persistent ones like "remember_me".

var express = require('express');
var app = express();

function handleRememberLogin(req, res) {
  var token = "remember-token-abc123";
  res.cookie('remember_me', token);
  res.json({ loggedIn: true });
}

function handleTracking(req, res) {
  var userId = req.body.userId;
  res.cookie('analytics_id', userId);
  res.json({ tracked: true });
}

app.post('/login/remember', handleRememberLogin);
app.post('/track', handleTracking);
