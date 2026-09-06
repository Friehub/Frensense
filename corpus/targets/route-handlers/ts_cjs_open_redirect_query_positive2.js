// [frensense]
// observation: The server uses res.redirect() with a URL taken directly from the req.query.url parameter without any validation, allowing an attacker to redirect users to arbitrary external domains.
// impact: An attacker can craft a phishing link that redirects users from the legitimate site to a malicious site, where credentials or session tokens can be stolen.
// improvement: Validate the redirect URL against an allowlist of trusted domains, or only allow relative redirects starting with '/'.

var express = require('express');
var app = express();

function loginCallback(req, res) {
  var destination = req.query.next;
  if (req.session) {
    req.session.authenticated = true;
  }
  res.redirect(destination);
}

function oauthReturn(req, res) {
  var code = req.query.code;
  var state = req.query.state;
  if (code && state) {
    res.redirect(req.query.returnUrl);
  } else {
    res.status(400).send('Missing OAuth parameters');
  }
}

app.get('/auth/callback', loginCallback);
app.get('/oauth/complete', oauthReturn);
