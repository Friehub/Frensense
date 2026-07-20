// [frensense]
// observation: The server uses res.redirect() with a URL taken directly from the req.query.url parameter without any validation, allowing an attacker to redirect users to arbitrary external domains.
// impact: An attacker can craft a phishing link that redirects users from the legitimate site to a malicious site, where credentials or session tokens can be stolen.
// improvement: Validate the redirect URL against an allowlist of trusted domains, or only allow relative redirects starting with '/'.

var express = require('express');
var app = express();

function logoutHandler(req, res) {
  var redirectTo = req.query.redirectTo;
  req.session.destroy(function(err) {
    if (err) return res.status(500).send('Logout failed');
    res.redirect(redirectTo);
  });
}

function postLoginRedirect(req, res) {
  var target = req.query.target || '/dashboard';
  if (req.body.rememberMe) {
    res.cookie('remember', 'true', { maxAge: 604800000 });
  }
  res.redirect(target);
}

app.get('/logout', logoutHandler);
app.post('/login', postLoginRedirect);
