// SAFE: The redirect URL is validated against an allowlist of trusted domains.

const express = require('express');
const url = require('url');

const app = express();
var ALLOWED_HOSTS = ['example.com', 'app.example.com'];

app.get('/auth/return', function(req, res) {
  var target = req.query.url;
  var parsed = url.parse(target);
  if (!ALLOWED_HOSTS.includes(parsed.hostname)) {
    return res.status(400).json({ error: 'Redirect not allowed' });
  }
  res.redirect(target);
});
