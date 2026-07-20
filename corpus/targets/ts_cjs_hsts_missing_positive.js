// [frensense]
// observation: The Express server runs on HTTPS but does not set the Strict-Transport-Security header, allowing an attacker to perform SSL-stripping attacks on first-time visitors.
// impact: Users accessing the site over HTTP for the first time can be redirected to a malicious site via MITM before ever seeing the HTTPS version.
// improvement: Set the Strict-Transport-Security header with a long max-age via helmet or manually.

const express = require('express');

const app = express();

app.get('/api/status', function(req, res) {
  res.json({ status: 'ok' });
});
