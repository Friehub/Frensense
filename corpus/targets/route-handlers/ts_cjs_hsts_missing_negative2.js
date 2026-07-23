// SAFE: Strict-Transport-Security header is set manually in a custom middleware.

const express = require('express');

const app = express();
app.use(function(req, res, next) {
  res.setHeader('Strict-Transport-Security', 'max-age=31536000; includeSubDomains');
  next();
});

app.get('/api/status', function(req, res) {
  res.json({ status: 'ok' });
});
