// SAFE: Only relative paths (starting with '/') are accepted for redirect, preventing external redirection.

const express = require('express');

const app = express();

app.get('/auth/return', function(req, res) {
  var target = req.query.url;
  if (typeof target !== 'string' || !target.startsWith('/')) {
    return res.status(400).json({ error: 'Only relative redirects allowed' });
  }
  res.redirect(target);
});
