// SAFE: Helmet's hsts middleware sets Strict-Transport-Security header with a 1-year max-age and includeSubDomains.

const express = require('express');
const helmet = require('helmet');

const app = express();
app.use(helmet.hsts({ maxAge: 31536000, includeSubDomains: true, preload: true }));

app.get('/api/status', function(req, res) {
  res.json({ status: 'ok' });
});
