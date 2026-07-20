// [frensense]
// observation: The server uses res.redirect() with a URL taken directly from the req.query.url parameter without any validation, allowing an attacker to redirect users to arbitrary external domains.
// impact: An attacker can craft a phishing link that redirects users from the legitimate site to a malicious site, where credentials or session tokens can be stolen.
// improvement: Validate the redirect URL against an allowlist of trusted domains, or only allow relative redirects starting with '/'.

const express = require('express');

const app = express();

app.get('/auth/return', function(req, res) {
  res.redirect(req.query.url);
});
